#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleDashboardChartData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    native_dashboard: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chart_layout: std::option::Option<std::vec::Vec<ChronicleDashboardChart_ChartLayout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_chart: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_query: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleDashboardChart_Timeouts>,
    dynamic: ChronicleDashboardChartDynamic,
}

struct ChronicleDashboardChart_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleDashboardChartData_>,
}

#[derive(Clone)]
pub struct ChronicleDashboardChart(std::rc::Rc<ChronicleDashboardChart_>);

impl ChronicleDashboardChart {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `native_dashboard`.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn set_native_dashboard(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().native_dashboard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chart_layout`.\n"]
    pub fn set_chart_layout(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_ChartLayout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chart_layout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chart_layout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dashboard_chart`.\n"]
    pub fn set_dashboard_chart(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_chart = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_chart = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dashboard_query`.\n"]
    pub fn set_dashboard_query(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleDashboardChart_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `chart_id` after provisioning.\nThe unique identifier of the chart, automatically extracted from the full resource name."]
    pub fn chart_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chart_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the DashboardChart."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `native_dashboard` after provisioning.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn native_dashboard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.native_dashboard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> terrars::ListRef<ChronicleDashboardChart_ChartLayoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chart_layout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_chart` after provisioning.\n"]
    pub fn dashboard_chart(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_chart", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_query` after provisioning.\n"]
    pub fn dashboard_query(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDashboardChart_TimeoutsRef {
        ChronicleDashboardChart_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleDashboardChart {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleDashboardChart { }

impl terrars::ToListMappable for ChronicleDashboardChart {
    type O = terrars::ListRef<ChronicleDashboardChartRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleDashboardChart_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_dashboard_chart".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleDashboardChart {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Chronicle instance."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc = "The location of the Chronicle instance."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleDashboardChart {
        let out = ChronicleDashboardChart(std::rc::Rc::new(ChronicleDashboardChart_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleDashboardChartData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                native_dashboard: core::default::Default::default(),
                project: core::default::Default::default(),
                chart_layout: core::default::Default::default(),
                dashboard_chart: core::default::Default::default(),
                dashboard_query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleDashboardChartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChartRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleDashboardChartRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chart_id` after provisioning.\nThe unique identifier of the chart, automatically extracted from the full resource name."]
    pub fn chart_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chart_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the DashboardChart."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `native_dashboard` after provisioning.\nThe parent NativeDashboard resource name, formatted as projects/{project}/locations/{location}/instances/{instance}/nativeDashboards/{dashboard_id}"]
    pub fn native_dashboard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.native_dashboard", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> terrars::ListRef<ChronicleDashboardChart_ChartLayoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chart_layout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_chart` after provisioning.\n"]
    pub fn dashboard_chart(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_chart", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_query` after provisioning.\n"]
    pub fn dashboard_query(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDashboardChart_TimeoutsRef {
        ChronicleDashboardChart_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChartDynamic {
    chart_layout: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_ChartLayout>>,
    dashboard_chart: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart>>,
    dashboard_query: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardQuery>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_ChartLayout {
    span_x: terrars::PrimField<f64>,
    span_y: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_x: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_y: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_ChartLayout {
    #[doc = "Set the field `start_x`.\n"]
    pub fn set_start_x(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_y`.\n"]
    pub fn set_start_y(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_ChartLayout {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_ChartLayout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_ChartLayout {
    #[doc = ""]
    pub span_x: terrars::PrimField<f64>,
    #[doc = ""]
    pub span_y: terrars::PrimField<f64>,
}

impl BuildChronicleDashboardChart_ChartLayout {
    pub fn build(self) -> ChronicleDashboardChart_ChartLayout {
        ChronicleDashboardChart_ChartLayout {
            span_x: self.span_x,
            span_y: self.span_y,
            start_x: core::default::Default::default(),
            start_y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_ChartLayoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_ChartLayoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDashboardChart_ChartLayoutRef {
        ChronicleDashboardChart_ChartLayoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_ChartLayoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `span_x` after provisioning.\n"]
    pub fn span_x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.span_x", self.base))
    }

    #[doc = "Get a reference to the value of field `span_y` after provisioning.\n"]
    pub fn span_y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.span_y", self.base))
    }

    #[doc = "Get a reference to the value of field `start_x` after provisioning.\n"]
    pub fn start_x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_x", self.base))
    }

    #[doc = "Get a reference to the value of field `start_y` after provisioning.\n"]
    pub fn start_y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_y", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_ChartDatasource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_sources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ChronicleDashboardChart_DashboardChart_ChartDatasource {
    #[doc =
        "Set the field `data_sources`.\nName(s) of the datasource used in the chart. Available values include:\n'UDM', 'ENTITY', 'INGESTION_METRICS', 'RULE_DETECTIONS', 'RULESETS',\n'GLOBAL', 'IOC_MATCHES', 'RULES', 'SOAR_CASES', 'SOAR_PLAYBOOKS',\n'SOAR_CASE_HISTORY', 'DATA_TABLE', 'INVESTIGATION', 'INVESTIGATION_FEEDBACK'."]
    pub fn set_data_sources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_sources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_ChartDatasource {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_ChartDatasource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_ChartDatasource {}

impl BuildChronicleDashboardChart_DashboardChart_ChartDatasource {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_ChartDatasource {
        ChronicleDashboardChart_DashboardChart_ChartDatasource { data_sources: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_ChartDatasourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_ChartDatasourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_ChartDatasourceRef {
        ChronicleDashboardChart_DashboardChart_ChartDatasourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_ChartDatasourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dashboard_query` after provisioning.\nThe unique system ID of the query linked to this chart."]
    pub fn dashboard_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_sources` after provisioning.\nName(s) of the datasource used in the chart. Available values include:\n'UDM', 'ENTITY', 'INGESTION_METRICS', 'RULE_DETECTIONS', 'RULESETS',\n'GLOBAL', 'IOC_MATCHES', 'RULES', 'SOAR_CASES', 'SOAR_PLAYBOOKS',\n'SOAR_CASE_HISTORY', 'DATA_TABLE', 'INVESTIGATION', 'INVESTIGATION_FEEDBACK'."]
    pub fn data_sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_sources", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    link: terrars::PrimField<std::string::String>,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
    type O =
        terrars
        ::BlockAssignable<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
    #[doc = ""]
    pub link: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
    pub fn build(
        self,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink {
            description: core::default::Default::default(),
            link: self.link,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `link` after provisioning.\n"]
    pub fn link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.link", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
    #[doc = "Set the field `field_values`.\n"]
    pub fn set_field_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_values = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_operator`.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn set_filter_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
    type O =
        terrars
        ::BlockAssignable<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
    pub fn build(
        self,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues {
            field_values: core::default::Default::default(),
            filter_operator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_values` after provisioning.\n"]
    pub fn field_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_operator` after provisioning.\n Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn filter_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_operator", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersDynamic {
    filter_operator_and_values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
    dashboard_filter_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operator_and_values: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues,
        >,
    >,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
    #[doc = "Set the field `filter_operator_and_values`.\n"]
    pub fn set_filter_operator_and_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValues,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_operator_and_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_operator_and_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
    type O =
        terrars
        ::BlockAssignable<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
    #[doc = ""]
    pub dashboard_filter_id: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
    pub fn build(
        self,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters {
            dashboard_filter_id: self.dashboard_filter_id,
            filter_operator_and_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dashboard_filter_id` after provisioning.\n"]
    pub fn dashboard_filter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_filter_id", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_operator_and_values` after provisioning.\n"]
    pub fn filter_operator_and_values(
        &self,
    ) -> terrars
    ::ListRef<
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters_FilterOperatorAndValuesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_operator_and_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterDynamic {
    dashboard_filters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_filters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters,
        >,
    >,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
    #[doc = "Set the field `dashboard_filters`.\n"]
    pub fn set_dashboard_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFilters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dashboard_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dashboard_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
    type O =
        terrars
        ::BlockAssignable<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
    pub fn build(
        self,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter {
            dashboard_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dashboard_filters` after provisioning.\n"]
    pub fn dashboard_filters(
        &self,
    ) -> terrars
    ::ListRef<
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter_DashboardFiltersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
    query: terrars::PrimField<std::string::String>,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query { }

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
    #[doc = ""]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query {
            query: self.query,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsDynamic {
    external_link: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink,
        >,
    >,
    filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter>,
    >,
    query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_click_column: std::option::Option<terrars::PrimField<std::string::String>>,
    new_tab: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_link: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query>,
    >,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
    #[doc = "Set the field `left_click_column`.\n"]
    pub fn set_left_click_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.left_click_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_link`.\n"]
    pub fn set_external_link(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_link = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_link = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
    #[doc = ""]
    pub new_tab: terrars::PrimField<bool>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings {
            left_click_column: core::default::Default::default(),
            new_tab: self.new_tab,
            external_link: core::default::Default::default(),
            filter: core::default::Default::default(),
            query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `left_click_column` after provisioning.\n"]
    pub fn left_click_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left_click_column", self.base))
    }

    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }

    #[doc = "Get a reference to the value of field `external_link` after provisioning.\n"]
    pub fn external_link(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_link", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
    enabled: terrars::PrimField<bool>,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings { }

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings {
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsDynamic {
    custom_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings>,
    >,
    default_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
    display_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings>>,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
    #[doc = "Set the field `custom_settings`.\n"]
    pub fn set_custom_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
    #[doc = ""]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns {
            display_name: self.display_name,
            id: self.id,
            custom_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_settings` after provisioning.\n"]
    pub fn custom_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsDynamic {
    external_link: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink,
        >,
    >,
    filter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter>,
    >,
    query: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
    new_tab: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_link: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query>,
    >,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
    #[doc = "Set the field `external_link`.\n"]
    pub fn set_external_link(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLink,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_link = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_link = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Filter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_Query,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
    #[doc = ""]
    pub new_tab: terrars::PrimField<bool>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings {
            new_tab: self.new_tab,
            external_link: core::default::Default::default(),
            filter: core::default::Default::default(),
            query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }

    #[doc = "Get a reference to the value of field `external_link` after provisioning.\n"]
    pub fn external_link(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_ExternalLinkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_link", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_CustomSettings_QueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsDynamic {
    custom_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings>,
    >,
    default_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
    display_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings>>,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
    #[doc = "Set the field `custom_settings`.\n"]
    pub fn set_custom_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
    #[doc = ""]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns {
            display_name: self.display_name,
            id: self.id,
            custom_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_settings` after provisioning.\n"]
    pub fn custom_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns_CustomSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns_DefaultSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_DrillDownConfigDynamic {
    left_drill_downs: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns>>,
    right_drill_downs: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_drill_downs: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right_drill_downs: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns>>,
    dynamic: ChronicleDashboardChart_DashboardChart_DrillDownConfigDynamic,
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfig {
    #[doc = "Set the field `left_drill_downs`.\n"]
    pub fn set_left_drill_downs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDowns>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.left_drill_downs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.left_drill_downs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `right_drill_downs`.\n"]
    pub fn set_right_drill_downs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDowns>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.right_drill_downs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.right_drill_downs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_DrillDownConfig {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_DrillDownConfig {}

impl BuildChronicleDashboardChart_DashboardChart_DrillDownConfig {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_DrillDownConfig {
        ChronicleDashboardChart_DashboardChart_DrillDownConfig {
            left_drill_downs: core::default::Default::default(),
            right_drill_downs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_DrillDownConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_DrillDownConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_DrillDownConfigRef {
        ChronicleDashboardChart_DashboardChart_DrillDownConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_DrillDownConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `left_drill_downs` after provisioning.\n"]
    pub fn left_drill_downs(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_LeftDrillDownsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.left_drill_downs", self.base))
    }

    #[doc = "Get a reference to the value of field `right_drill_downs` after provisioning.\n"]
    pub fn right_drill_downs(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfig_RightDrillDownsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.right_drill_downs", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
    #[doc =
        "Set the field `button_style`.\n Possible values: [\"BUTTON_STYLE_UNSPECIFIED\", \"BUTTON_STYLE_FILLED\", \"BUTTON_STYLE_OUTLINED\", \"BUTTON_STYLE_TRANSPARENT\"]"]
    pub fn set_button_style(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.button_style = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
        ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties {
            button_style: core::default::Default::default(),
            color: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `button_style` after provisioning.\n Possible values: [\"BUTTON_STYLE_UNSPECIFIED\", \"BUTTON_STYLE_FILLED\", \"BUTTON_STYLE_OUTLINED\", \"BUTTON_STYLE_TRANSPARENT\"]"]
    pub fn button_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_style", self.base))
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_ButtonDynamic {
    properties: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Button {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hyperlink: terrars::PrimField<std::string::String>,
    label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_tab: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_ButtonDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Button {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_tab`.\n"]
    pub fn set_new_tab(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.new_tab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Button_Properties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Button {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Button>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Button {
    #[doc = ""]
    pub hyperlink: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub label: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Button {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Button {
        ChronicleDashboardChart_DashboardChart_Visualization_Button {
            description: core::default::Default::default(),
            hyperlink: self.hyperlink,
            label: self.label,
            new_tab: core::default::Default::default(),
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef {
        ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `hyperlink` after provisioning.\n"]
    pub fn hyperlink(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hyperlink", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `new_tab` after provisioning.\n"]
    pub fn new_tab(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_tab", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Button_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
        ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs {
            field: core::default::Default::default(),
            header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count_column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude_column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude_column: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
    #[doc = "Set the field `count_column`.\n"]
    pub fn set_count_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.count_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latitude_column`.\n"]
    pub fn set_latitude_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.latitude_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `longitude_column`.\n"]
    pub fn set_longitude_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.longitude_column = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
    type O =
        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings {
            count_column: core::default::Default::default(),
            latitude_column: core::default::Default::default(),
            longitude_column: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count_column` after provisioning.\n"]
    pub fn count_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count_column", self.base))
    }

    #[doc = "Get a reference to the value of field `latitude_column` after provisioning.\n"]
    pub fn latitude_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude_column", self.base))
    }

    #[doc = "Get a reference to the value of field `longitude_column` after provisioning.\n"]
    pub fn longitude_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude_column", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fit_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zoom_scale_value: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
    #[doc = "Set the field `fit_data`.\n"]
    pub fn set_fit_data(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fit_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latitude_value`.\n"]
    pub fn set_latitude_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.latitude_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `longitude_value`.\n"]
    pub fn set_longitude_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.longitude_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zoom_scale_value`.\n"]
    pub fn set_zoom_scale_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.zoom_scale_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
    type O =
        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition {
            fit_data: core::default::Default::default(),
            latitude_value: core::default::Default::default(),
            longitude_value: core::default::Default::default(),
            zoom_scale_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fit_data` after provisioning.\n"]
    pub fn fit_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fit_data", self.base))
    }

    #[doc = "Get a reference to the value of field `latitude_value` after provisioning.\n"]
    pub fn latitude_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude_value", self.base))
    }

    #[doc = "Get a reference to the value of field `longitude_value` after provisioning.\n"]
    pub fn longitude_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude_value", self.base))
    }

    #[doc = "Get a reference to the value of field `zoom_scale_value` after provisioning.\n"]
    pub fn zoom_scale_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zoom_scale_value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_size_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `point_size_type`.\n Possible values: [\"POINT_SIZE_TYPE_UNSPECIFIED\", \"POINT_SIZE_TYPE_FIXED\", \"POINT_SIZE_TYPE_PROPORTIONAL_TO_SIZE\"]"]
    pub fn set_point_size_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.point_size_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings {
            color: core::default::Default::default(),
            point_size_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `point_size_type` after provisioning.\n Possible values: [\"POINT_SIZE_TYPE_UNSPECIFIED\", \"POINT_SIZE_TYPE_FIXED\", \"POINT_SIZE_TYPE_PROPORTIONAL_TO_SIZE\"]"]
    pub fn point_size_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.point_size_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigDynamic {
    data_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings>,
    >,
    map_position: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition>,
    >,
    point_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plot_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_position: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    point_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
    #[doc =
        "Set the field `plot_mode`.\n Possible values: [\"PLOT_MODE_UNSPECIFIED\", \"PLOT_MODE_POINTS\", \"PLOT_MODE_HEATMAP\", \"PLOT_MODE_BOTH\"]"]
    pub fn set_plot_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plot_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_settings`.\n"]
    pub fn set_data_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `map_position`.\n"]
    pub fn set_map_position(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPosition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.map_position = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.map_position = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `point_settings`.\n"]
    pub fn set_point_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.point_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.point_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig {
            plot_mode: core::default::Default::default(),
            data_settings: core::default::Default::default(),
            map_position: core::default::Default::default(),
            point_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef {
        ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `plot_mode` after provisioning.\n Possible values: [\"PLOT_MODE_UNSPECIFIED\", \"PLOT_MODE_POINTS\", \"PLOT_MODE_HEATMAP\", \"PLOT_MODE_BOTH\"]"]
    pub fn plot_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plot_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `data_settings` after provisioning.\n"]
    pub fn data_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_DataSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `map_position` after provisioning.\n"]
    pub fn map_position(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_MapPositionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.map_position", self.base))
    }

    #[doc = "Get a reference to the value of field `point_settings` after provisioning.\n"]
    pub fn point_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig_PointSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.point_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Legends {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bottom: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    legend_align: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    legend_orient: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    padding: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    z: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    z_level: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Legends {
    #[doc = "Set the field `bottom`.\n"]
    pub fn set_bottom(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bottom = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `left`.\n"]
    pub fn set_left(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.left = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `legend_align`.\n Possible values: [\"AUTO\", \"LEFT\", \"RIGHT\"]"]
    pub fn set_legend_align(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.legend_align = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `legend_orient`.\n Possible values: [\"VERTICAL\", \"HORIZONTAL\"]"]
    pub fn set_legend_orient(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.legend_orient = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `padding`.\n"]
    pub fn set_padding(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.padding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `right`.\n"]
    pub fn set_right(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.right = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top`.\n"]
    pub fn set_top(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `z`.\n"]
    pub fn set_z(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.z = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `z_level`.\n"]
    pub fn set_z_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.z_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Legends {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Legends>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Legends {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Legends {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Legends {
        ChronicleDashboardChart_DashboardChart_Visualization_Legends {
            bottom: core::default::Default::default(),
            id: core::default::Default::default(),
            left: core::default::Default::default(),
            legend_align: core::default::Default::default(),
            legend_orient: core::default::Default::default(),
            padding: core::default::Default::default(),
            right: core::default::Default::default(),
            show: core::default::Default::default(),
            top: core::default::Default::default(),
            z: core::default::Default::default(),
            z_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bottom` after provisioning.\n"]
    pub fn bottom(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bottom", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `left` after provisioning.\n"]
    pub fn left(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.left", self.base))
    }

    #[doc =
        "Get a reference to the value of field `legend_align` after provisioning.\n Possible values: [\"AUTO\", \"LEFT\", \"RIGHT\"]"]
    pub fn legend_align(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.legend_align", self.base))
    }

    #[doc =
        "Get a reference to the value of field `legend_orient` after provisioning.\n Possible values: [\"VERTICAL\", \"HORIZONTAL\"]"]
    pub fn legend_orient(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.legend_orient", self.base))
    }

    #[doc = "Get a reference to the value of field `padding` after provisioning.\n"]
    pub fn padding(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.padding", self.base))
    }

    #[doc = "Get a reference to the value of field `right` after provisioning.\n"]
    pub fn right(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.right", self.base))
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }

    #[doc = "Get a reference to the value of field `top` after provisioning.\n"]
    pub fn top(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top", self.base))
    }

    #[doc = "Get a reference to the value of field `z` after provisioning.\n"]
    pub fn z(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.z", self.base))
    }

    #[doc = "Get a reference to the value of field `z_level` after provisioning.\n"]
    pub fn z_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.z_level", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    background_color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
    #[doc = "Set the field `background_color`.\n"]
    pub fn set_background_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.background_color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
        ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties {
            background_color: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `background_color` after provisioning.\n"]
    pub fn background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.background_color", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_MarkdownDynamic {
    properties: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Markdown {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_MarkdownDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Markdown {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Markdown_Properties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Markdown {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Markdown>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Markdown {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Markdown {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Markdown {
        ChronicleDashboardChart_DashboardChart_Visualization_Markdown {
            content: self.content,
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef {
        ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Markdown_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_blur: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_offset_x: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shadow_offset_y: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opacity`.\n"]
    pub fn set_opacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.opacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_blur`.\n"]
    pub fn set_shadow_blur(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_blur = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_color`.\n"]
    pub fn set_shadow_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shadow_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_offset_x`.\n"]
    pub fn set_shadow_offset_x(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_offset_x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shadow_offset_y`.\n"]
    pub fn set_shadow_offset_y(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shadow_offset_y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle {
            color: core::default::Default::default(),
            opacity: core::default::Default::default(),
            origin: core::default::Default::default(),
            shadow_blur: core::default::Default::default(),
            shadow_color: core::default::Default::default(),
            shadow_offset_x: core::default::Default::default(),
            shadow_offset_y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `opacity` after provisioning.\n"]
    pub fn opacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opacity", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_blur` after provisioning.\n"]
    pub fn shadow_blur(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_blur", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_color` after provisioning.\n"]
    pub fn shadow_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_color", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_offset_x` after provisioning.\n"]
    pub fn shadow_offset_x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_offset_x", self.base))
    }

    #[doc = "Get a reference to the value of field `shadow_offset_y` after provisioning.\n"]
    pub fn shadow_offset_y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shadow_offset_y", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
    #[doc = "Set the field `show`.\nWhether to show data label."]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel {
            show: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\nWhether to show data label."]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
    #[doc = "Set the field `item_name`.\n"]
    pub fn set_item_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.item_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x`.\n"]
    pub fn set_x(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `y`.\n"]
    pub fn set_y(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode {
            item_name: core::default::Default::default(),
            value: core::default::Default::default(),
            x: core::default::Default::default(),
            y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `item_name` after provisioning.\n"]
    pub fn item_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `x` after provisioning.\n"]
    pub fn x(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x", self.base))
    }

    #[doc = "Get a reference to the value of field `y` after provisioning.\n"]
    pub fn y(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.y", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
    type O =
        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue {
            color: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
    type O =
        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue {
            color: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues {
            color: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigDynamic {
    base_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue>,
    >,
    limit_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue>,
    >,
    threshold_values: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_value: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_value: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_values: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
    #[doc = "Set the field `base_value`.\n"]
    pub fn set_base_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `limit_value`.\n"]
    pub fn set_limit_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limit_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limit_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threshold_values`.\n"]
    pub fn set_threshold_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValues,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threshold_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threshold_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig {
            base_value: core::default::Default::default(),
            limit_value: core::default::Default::default(),
            threshold_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_value` after provisioning.\n"]
    pub fn base_value(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_BaseValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_value", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_value` after provisioning.\n"]
    pub fn limit_value(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_LimitValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limit_value", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_values` after provisioning.\n"]
    pub fn threshold_values(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig_ThresholdValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_values", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value {
            color: core::default::Default::default(),
            label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsDynamic {
    value: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_Value,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsDynamic {
    colors: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    colors: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
    #[doc = "Set the field `colors`.\n"]
    pub fn set_colors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_Colors,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.colors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.colors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors {
            colors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `colors` after provisioning.\n"]
    pub fn colors(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors_ColorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.colors", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    border_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    border_width: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
    #[doc = "Set the field `border_color`.\n"]
    pub fn set_border_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.border_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `border_width`.\n"]
    pub fn set_border_width(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.border_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle {
            border_color: core::default::Default::default(),
            border_width: core::default::Default::default(),
            color: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `border_color` after provisioning.\n"]
    pub fn border_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.border_color", self.base))
    }

    #[doc = "Get a reference to the value of field `border_width` after provisioning.\n"]
    pub fn border_width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.border_width", self.base))
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_display_trend: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_trend_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_metric_trend: std::option::Option<terrars::PrimField<bool>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
    #[doc =
        "Set the field `metric_display_trend`.\n Possible values: [\"METRIC_DISPLAY_TREND_UNSPECIFIED\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE\", \"METRIC_DISPLAY_TREND_PERCENTAGE\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE_AND_PERCENTAGE\"]"]
    pub fn set_metric_display_trend(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_display_trend = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metric_format`.\n Possible values: [\"METRIC_FORMAT_UNSPECIFIED\", \"METRIC_FORMAT_NUMBER\", \"METRIC_FORMAT_PLAIN_TEXT\"]"]
    pub fn set_metric_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metric_trend_type`.\n Possible values: [\"METRIC_TREND_TYPE_UNSPECIFIED\", \"METRIC_TREND_TYPE_REGULAR\", \"METRIC_TREND_TYPE_INVERSE\"]"]
    pub fn set_metric_trend_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_trend_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_metric_trend`.\n"]
    pub fn set_show_metric_trend(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show_metric_trend = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig {
            metric_display_trend: core::default::Default::default(),
            metric_format: core::default::Default::default(),
            metric_trend_type: core::default::Default::default(),
            show_metric_trend: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef {
        ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metric_display_trend` after provisioning.\n Possible values: [\"METRIC_DISPLAY_TREND_UNSPECIFIED\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE\", \"METRIC_DISPLAY_TREND_PERCENTAGE\", \"METRIC_DISPLAY_TREND_ABSOLUTE_VALUE_AND_PERCENTAGE\"]"]
    pub fn metric_display_trend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_display_trend", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_format` after provisioning.\n Possible values: [\"METRIC_FORMAT_UNSPECIFIED\", \"METRIC_FORMAT_NUMBER\", \"METRIC_FORMAT_PLAIN_TEXT\"]"]
    pub fn metric_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_trend_type` after provisioning.\n Possible values: [\"METRIC_TREND_TYPE_UNSPECIFIED\", \"METRIC_TREND_TYPE_REGULAR\", \"METRIC_TREND_TYPE_INVERSE\"]"]
    pub fn metric_trend_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_trend_type", self.base))
    }

    #[doc = "Get a reference to the value of field `show_metric_trend` after provisioning.\n"]
    pub fn show_metric_trend(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_metric_trend", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_SeriesDynamic {
    area_style: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle>>,
    data_label: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel>>,
    encode: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode>>,
    gauge_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig>>,
    item_colors: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors>>,
    item_style: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle>>,
    metric_trend_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Series {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    radius: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series_stack_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series_unique_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_background: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_symbol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    area_style: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_label: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encode: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gauge_config: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_colors: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_style: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_trend_config: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_SeriesDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Series {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `radius`.\n"]
    pub fn set_radius(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.radius = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `series_name`.\nUser specified series label."]
    pub fn set_series_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.series_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `series_stack_strategy`.\n Possible values: [\"SAMESIGN\", \"ALL\", \"POSITIVE\", \"NEGATIVE\"]"]
    pub fn set_series_stack_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.series_stack_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `series_type`.\n Possible values: [\"LINE\", \"BAR\", \"PIE\", \"TEXT\", \"MAP\", \"GAUGE\", \"SCATTERPLOT\"]"]
    pub fn set_series_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.series_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `series_unique_value`.\n"]
    pub fn set_series_unique_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.series_unique_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_background`.\n"]
    pub fn set_show_background(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show_background = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_symbol`.\n"]
    pub fn set_show_symbol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show_symbol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack`.\n"]
    pub fn set_stack(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `area_style`.\n"]
    pub fn set_area_style(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyle>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.area_style = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.area_style = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_label`.\n"]
    pub fn set_data_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabel>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encode`.\n"]
    pub fn set_encode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_Encode>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gauge_config`.\n"]
    pub fn set_gauge_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gauge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gauge_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `item_colors`.\n"]
    pub fn set_item_colors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColors>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.item_colors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.item_colors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `item_style`.\n"]
    pub fn set_item_style(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyle>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.item_style = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.item_style = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_trend_config`.\n"]
    pub fn set_metric_trend_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_trend_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_trend_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Series {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Series {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Series {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Series {
        ChronicleDashboardChart_DashboardChart_Visualization_Series {
            field: core::default::Default::default(),
            label: core::default::Default::default(),
            radius: core::default::Default::default(),
            series_name: core::default::Default::default(),
            series_stack_strategy: core::default::Default::default(),
            series_type: core::default::Default::default(),
            series_unique_value: core::default::Default::default(),
            show_background: core::default::Default::default(),
            show_symbol: core::default::Default::default(),
            stack: core::default::Default::default(),
            area_style: core::default::Default::default(),
            data_label: core::default::Default::default(),
            encode: core::default::Default::default(),
            gauge_config: core::default::Default::default(),
            item_colors: core::default::Default::default(),
            item_style: core::default::Default::default(),
            metric_trend_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `radius` after provisioning.\n"]
    pub fn radius(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.radius", self.base))
    }

    #[doc = "Get a reference to the value of field `series_name` after provisioning.\nUser specified series label."]
    pub fn series_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.series_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `series_stack_strategy` after provisioning.\n Possible values: [\"SAMESIGN\", \"ALL\", \"POSITIVE\", \"NEGATIVE\"]"]
    pub fn series_stack_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.series_stack_strategy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `series_type` after provisioning.\n Possible values: [\"LINE\", \"BAR\", \"PIE\", \"TEXT\", \"MAP\", \"GAUGE\", \"SCATTERPLOT\"]"]
    pub fn series_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.series_type", self.base))
    }

    #[doc = "Get a reference to the value of field `series_unique_value` after provisioning.\n"]
    pub fn series_unique_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.series_unique_value", self.base))
    }

    #[doc = "Get a reference to the value of field `show_background` after provisioning.\n"]
    pub fn show_background(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_background", self.base))
    }

    #[doc = "Get a reference to the value of field `show_symbol` after provisioning.\n"]
    pub fn show_symbol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_symbol", self.base))
    }

    #[doc = "Get a reference to the value of field `stack` after provisioning.\n"]
    pub fn stack(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack", self.base))
    }

    #[doc = "Get a reference to the value of field `area_style` after provisioning.\n"]
    pub fn area_style(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_AreaStyleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.area_style", self.base))
    }

    #[doc = "Get a reference to the value of field `data_label` after provisioning.\n"]
    pub fn data_label(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_DataLabelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_label", self.base))
    }

    #[doc = "Get a reference to the value of field `encode` after provisioning.\n"]
    pub fn encode(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_EncodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encode", self.base))
    }

    #[doc = "Get a reference to the value of field `gauge_config` after provisioning.\n"]
    pub fn gauge_config(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_GaugeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gauge_config", self.base))
    }

    #[doc = "Get a reference to the value of field `item_colors` after provisioning.\n"]
    pub fn item_colors(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemColorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.item_colors", self.base))
    }

    #[doc = "Get a reference to the value of field `item_style` after provisioning.\n"]
    pub fn item_style(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_ItemStyleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.item_style", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_trend_config` after provisioning.\n"]
    pub fn metric_trend_config(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_Series_MetricTrendConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_trend_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_render_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
    #[doc =
        "Set the field `column_render_type`.\n Possible values: [\"RENDER_TYPE_UNSPECIFIED\", \"RENDER_TYPE_TEXT\", \"RENDER_TYPE_ICON\", \"RENDER_TYPE_ICON_AND_TEXT\"]"]
    pub fn set_column_render_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.column_render_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings {
            column_render_type: core::default::Default::default(),
            field: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `column_render_type` after provisioning.\n Possible values: [\"RENDER_TYPE_UNSPECIFIED\", \"RENDER_TYPE_TEXT\", \"RENDER_TYPE_ICON\", \"RENDER_TYPE_ICON_AND_TEXT\"]"]
    pub fn column_render_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_render_type", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cell_tooltip_text: std::option::Option<terrars::PrimField<std::string::String>>,
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_tooltip_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
    #[doc = "Set the field `cell_tooltip_text`.\n"]
    pub fn set_cell_tooltip_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cell_tooltip_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_tooltip_text`.\n"]
    pub fn set_header_tooltip_text(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.header_tooltip_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
    type O =
        terrars
        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings {
            cell_tooltip_text: core::default::Default::default(),
            field: self.field,
            header_tooltip_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cell_tooltip_text` after provisioning.\n"]
    pub fn cell_tooltip_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cell_tooltip_text", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `header_tooltip_text` after provisioning.\n"]
    pub fn header_tooltip_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_tooltip_text", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfigDynamic {
    column_render_type_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings>,
    >,
    column_tooltip_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_text_wrap: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_render_type_settings: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_tooltip_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_TableConfigDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
    #[doc = "Set the field `enable_text_wrap`.\n"]
    pub fn set_enable_text_wrap(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_text_wrap = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_render_type_settings`.\n"]
    pub fn set_column_render_type_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_render_type_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_render_type_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `column_tooltip_settings`.\n"]
    pub fn set_column_tooltip_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_tooltip_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_tooltip_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfig {
            enable_text_wrap: core::default::Default::default(),
            column_render_type_settings: core::default::Default::default(),
            column_tooltip_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef {
        ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_text_wrap` after provisioning.\n"]
    pub fn enable_text_wrap(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_text_wrap", self.base))
    }

    #[doc = "Get a reference to the value of field `column_render_type_settings` after provisioning.\n"]
    pub fn column_render_type_settings(
        &self,
    ) -> terrars
    ::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnRenderTypeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_render_type_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `column_tooltip_settings` after provisioning.\n"]
    pub fn column_tooltip_settings(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig_ColumnTooltipSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_tooltip_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tooltip_trigger: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
    #[doc = "Set the field `show`.\n"]
    pub fn set_show(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.show = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tooltip_trigger`.\n Possible values: [\"TOOLTIP_TRIGGER_UNSPECIFIED\", \"TOOLTIP_TRIGGER_NONE\", \"TOOLTIP_TRIGGER_ITEM\", \"TOOLTIP_TRIGGER_AXIS\"]"]
    pub fn set_tooltip_trigger(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tooltip_trigger = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Tooltip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_Tooltip {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
        ChronicleDashboardChart_DashboardChart_Visualization_Tooltip {
            show: core::default::Default::default(),
            tooltip_trigger: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef {
        ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `show` after provisioning.\n"]
    pub fn show(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tooltip_trigger` after provisioning.\n Possible values: [\"TOOLTIP_TRIGGER_UNSPECIFIED\", \"TOOLTIP_TRIGGER_NONE\", \"TOOLTIP_TRIGGER_ITEM\", \"TOOLTIP_TRIGGER_AXIS\"]"]
    pub fn tooltip_trigger(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tooltip_trigger", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
    #[doc = "Set the field `color`.\n"]
    pub fn set_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
        ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces {
            color: core::default::Default::default(),
            label: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsDynamic {
    pieces: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visual_map_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pieces: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces>>,
    dynamic: ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
    #[doc =
        "Set the field `visual_map_type`.\n Possible values: [\"VISUAL_MAP_TYPE_UNSPECIFIED\", \"CONTINUOUS\", \"PIECEWISE\"]"]
    pub fn set_visual_map_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visual_map_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pieces`.\n"]
    pub fn set_pieces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_Pieces>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pieces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pieces = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
        ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps {
            visual_map_type: core::default::Default::default(),
            pieces: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef {
        ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `visual_map_type` after provisioning.\n Possible values: [\"VISUAL_MAP_TYPE_UNSPECIFIED\", \"CONTINUOUS\", \"PIECEWISE\"]"]
    pub fn visual_map_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visual_map_type", self.base))
    }

    #[doc = "Get a reference to the value of field `pieces` after provisioning.\n"]
    pub fn pieces(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps_PiecesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pieces", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_XAxes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    axis_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_XAxes {
    #[doc = "Set the field `axis_type`.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn set_axis_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.axis_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_XAxes {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_XAxes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_XAxes {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_XAxes {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_XAxes {
        ChronicleDashboardChart_DashboardChart_Visualization_XAxes {
            axis_type: core::default::Default::default(),
            display_name: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `axis_type` after provisioning.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn axis_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.axis_type", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_YAxes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    axis_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleDashboardChart_DashboardChart_Visualization_YAxes {
    #[doc = "Set the field `axis_type`.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn set_axis_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.axis_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization_YAxes {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_YAxes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization_YAxes {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization_YAxes {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization_YAxes {
        ChronicleDashboardChart_DashboardChart_Visualization_YAxes {
            axis_type: core::default::Default::default(),
            display_name: core::default::Default::default(),
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef {
        ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `axis_type` after provisioning.\n Possible values: [\"VALUE\", \"CATEGORY\", \"TIME\", \"LOG\"]"]
    pub fn axis_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.axis_type", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChart_VisualizationDynamic {
    button: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Button>>,
    column_defs: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs>>,
    google_maps_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig>>,
    legends: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Legends>>,
    markdown: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Markdown>>,
    series: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Series>>,
    table_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig>>,
    tooltip: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_Tooltip>>,
    visual_maps: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps>>,
    x_axes: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_XAxes>>,
    y_axes: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization_YAxes>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart_Visualization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grouping_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series_column: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_coloring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Button>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_defs: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_maps_config: std
    ::option
    ::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    legends: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Legends>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    markdown: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Markdown>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    series: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Series>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_config: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tooltip: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_Tooltip>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visual_maps: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x_axes: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_XAxes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    y_axes: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization_YAxes>>,
    dynamic: ChronicleDashboardChart_DashboardChart_VisualizationDynamic,
}

impl ChronicleDashboardChart_DashboardChart_Visualization {
    #[doc = "Set the field `grouping_type`.\n"]
    pub fn set_grouping_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grouping_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `series_column`.\n"]
    pub fn set_series_column(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.series_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_coloring_enabled`.\n"]
    pub fn set_threshold_coloring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.threshold_coloring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `button`.\n"]
    pub fn set_button(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Button>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.button = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.button = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `column_defs`.\n"]
    pub fn set_column_defs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_defs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_defs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_maps_config`.\n"]
    pub fn set_google_maps_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_maps_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_maps_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `legends`.\n"]
    pub fn set_legends(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Legends>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.legends = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.legends = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `markdown`.\n"]
    pub fn set_markdown(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Markdown>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.markdown = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.markdown = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `series`.\n"]
    pub fn set_series(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Series>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.series = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.series = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_config`.\n"]
    pub fn set_table_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_TableConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tooltip`.\n"]
    pub fn set_tooltip(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_Tooltip>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tooltip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tooltip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `visual_maps`.\n"]
    pub fn set_visual_maps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_VisualMaps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.visual_maps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.visual_maps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `x_axes`.\n"]
    pub fn set_x_axes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_XAxes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.x_axes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.x_axes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `y_axes`.\n"]
    pub fn set_y_axes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization_YAxes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.y_axes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.y_axes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart_Visualization {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart_Visualization {}

impl BuildChronicleDashboardChart_DashboardChart_Visualization {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart_Visualization {
        ChronicleDashboardChart_DashboardChart_Visualization {
            grouping_type: core::default::Default::default(),
            series_column: core::default::Default::default(),
            threshold_coloring_enabled: core::default::Default::default(),
            button: core::default::Default::default(),
            column_defs: core::default::Default::default(),
            google_maps_config: core::default::Default::default(),
            legends: core::default::Default::default(),
            markdown: core::default::Default::default(),
            series: core::default::Default::default(),
            table_config: core::default::Default::default(),
            tooltip: core::default::Default::default(),
            visual_maps: core::default::Default::default(),
            x_axes: core::default::Default::default(),
            y_axes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChart_VisualizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChart_VisualizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardChart_VisualizationRef {
        ChronicleDashboardChart_DashboardChart_VisualizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChart_VisualizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grouping_type` after provisioning.\n"]
    pub fn grouping_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grouping_type", self.base))
    }

    #[doc = "Get a reference to the value of field `series_column` after provisioning.\n"]
    pub fn series_column(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.series_column", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_coloring_enabled` after provisioning.\n"]
    pub fn threshold_coloring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_coloring_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `button` after provisioning.\n"]
    pub fn button(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_ButtonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.button", self.base))
    }

    #[doc = "Get a reference to the value of field `column_defs` after provisioning.\n"]
    pub fn column_defs(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_ColumnDefsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_defs", self.base))
    }

    #[doc = "Get a reference to the value of field `google_maps_config` after provisioning.\n"]
    pub fn google_maps_config(
        &self,
    ) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_GoogleMapsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_maps_config", self.base))
    }

    #[doc = "Get a reference to the value of field `legends` after provisioning.\n"]
    pub fn legends(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_LegendsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.legends", self.base))
    }

    #[doc = "Get a reference to the value of field `markdown` after provisioning.\n"]
    pub fn markdown(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_MarkdownRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.markdown", self.base))
    }

    #[doc = "Get a reference to the value of field `series` after provisioning.\n"]
    pub fn series(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_SeriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.series", self.base))
    }

    #[doc = "Get a reference to the value of field `table_config` after provisioning.\n"]
    pub fn table_config(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_TableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tooltip` after provisioning.\n"]
    pub fn tooltip(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_TooltipRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tooltip", self.base))
    }

    #[doc = "Get a reference to the value of field `visual_maps` after provisioning.\n"]
    pub fn visual_maps(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_VisualMapsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visual_maps", self.base))
    }

    #[doc = "Get a reference to the value of field `x_axes` after provisioning.\n"]
    pub fn x_axes(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_XAxesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x_axes", self.base))
    }

    #[doc = "Get a reference to the value of field `y_axes` after provisioning.\n"]
    pub fn y_axes(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_Visualization_YAxesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.y_axes", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardChartDynamic {
    chart_datasource: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_ChartDatasource>>,
    drill_down_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_DrillDownConfig>>,
    visualization: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardChart_Visualization>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardChart {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tile_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chart_datasource: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_ChartDatasource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drill_down_config: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_DrillDownConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visualization: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardChart_Visualization>>,
    dynamic: ChronicleDashboardChart_DashboardChartDynamic,
}

impl ChronicleDashboardChart_DashboardChart {
    #[doc = "Set the field `description`.\nDescription of the dashboardChart."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tile_type`.\nType of tile (e.g., visualization, button, markdown). Possible values: [\"TILE_TYPE_UNSPECIFIED\", \"TILE_TYPE_VISUALIZATION\", \"TILE_TYPE_BUTTON\", \"TILE_TYPE_MARKDOWN\"]"]
    pub fn set_tile_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tile_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chart_datasource`.\n"]
    pub fn set_chart_datasource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_ChartDatasource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chart_datasource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chart_datasource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `drill_down_config`.\n"]
    pub fn set_drill_down_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_DrillDownConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.drill_down_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.drill_down_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `visualization`.\n"]
    pub fn set_visualization(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart_Visualization>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.visualization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.visualization = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardChart {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardChart>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardChart {
    #[doc = "Display name/Title of the dashboardChart visible to users."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardChart {
    pub fn build(self) -> ChronicleDashboardChart_DashboardChart {
        ChronicleDashboardChart_DashboardChart {
            description: core::default::Default::default(),
            display_name: self.display_name,
            tile_type: core::default::Default::default(),
            chart_datasource: core::default::Default::default(),
            drill_down_config: core::default::Default::default(),
            visualization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardChartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardChartRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDashboardChart_DashboardChartRef {
        ChronicleDashboardChart_DashboardChartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardChartRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the dashboardChart."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name/Title of the dashboardChart visible to users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the DashboardChart."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tile_type` after provisioning.\nType of tile (e.g., visualization, button, markdown). Possible values: [\"TILE_TYPE_UNSPECIFIED\", \"TILE_TYPE_VISUALIZATION\", \"TILE_TYPE_BUTTON\", \"TILE_TYPE_MARKDOWN\"]"]
    pub fn tile_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tile_type", self.base))
    }

    #[doc = "Get a reference to the value of field `chart_datasource` after provisioning.\n"]
    pub fn chart_datasource(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_ChartDatasourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chart_datasource", self.base))
    }

    #[doc = "Get a reference to the value of field `drill_down_config` after provisioning.\n"]
    pub fn drill_down_config(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_DrillDownConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drill_down_config", self.base))
    }

    #[doc = "Get a reference to the value of field `visualization` after provisioning.\n"]
    pub fn visualization(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardChart_VisualizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visualization", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
    start_time_val: terrars::PrimField<std::string::String>,
    time_unit: terrars::PrimField<std::string::String>,
}

impl ChronicleDashboardChart_DashboardQuery_Input_RelativeTime { }

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input_RelativeTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
    #[doc = ""]
    pub start_time_val: terrars::PrimField<std::string::String>,
    #[doc =
        "The time unit for the relative range. Possible values: [\"SECOND\", \"MINUTE\", \"HOUR\", \"DAY\", \"WEEK\", \"MONTH\", \"YEAR\"]"]
    pub time_unit: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
    pub fn build(self) -> ChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
        ChronicleDashboardChart_DashboardQuery_Input_RelativeTime {
            start_time_val: self.start_time_val,
            time_unit: self.time_unit,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef {
        ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `start_time_val` after provisioning.\n"]
    pub fn start_time_val(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time_val", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_unit` after provisioning.\nThe time unit for the relative range. Possible values: [\"SECOND\", \"MINUTE\", \"HOUR\", \"DAY\", \"WEEK\", \"MONTH\", \"YEAR\"]"]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input_TimeWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardQuery_Input_TimeWindow {}

impl BuildChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
    pub fn build(self) -> ChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
        ChronicleDashboardChart_DashboardQuery_Input_TimeWindow {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef {
        ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardQuery_InputDynamic {
    relative_time: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardQuery_Input_RelativeTime>>,
    time_window: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardQuery_Input_TimeWindow>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardQuery_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_time: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardQuery_Input_RelativeTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_window: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardQuery_Input_TimeWindow>>,
    dynamic: ChronicleDashboardChart_DashboardQuery_InputDynamic,
}

impl ChronicleDashboardChart_DashboardQuery_Input {
    #[doc = "Set the field `relative_time`.\n"]
    pub fn set_relative_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input_RelativeTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.relative_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.relative_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_window`.\n"]
    pub fn set_time_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input_TimeWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardQuery_Input {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardQuery_Input {}

impl BuildChronicleDashboardChart_DashboardQuery_Input {
    pub fn build(self) -> ChronicleDashboardChart_DashboardQuery_Input {
        ChronicleDashboardChart_DashboardQuery_Input {
            relative_time: core::default::Default::default(),
            time_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardQuery_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardQuery_InputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDashboardChart_DashboardQuery_InputRef {
        ChronicleDashboardChart_DashboardQuery_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardQuery_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `relative_time` after provisioning.\n"]
    pub fn relative_time(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardQuery_Input_RelativeTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relative_time", self.base))
    }

    #[doc = "Get a reference to the value of field `time_window` after provisioning.\n"]
    pub fn time_window(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardQuery_Input_TimeWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_window", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDashboardChart_DashboardQueryDynamic {
    input: std::option::Option<terrars::DynamicBlock<ChronicleDashboardChart_DashboardQuery_Input>>,
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_DashboardQuery {
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<ChronicleDashboardChart_DashboardQuery_Input>>,
    dynamic: ChronicleDashboardChart_DashboardQueryDynamic,
}

impl ChronicleDashboardChart_DashboardQuery {
    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery_Input>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDashboardChart_DashboardQuery {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_DashboardQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_DashboardQuery {
    #[doc = "The raw query string."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDashboardChart_DashboardQuery {
    pub fn build(self) -> ChronicleDashboardChart_DashboardQuery {
        ChronicleDashboardChart_DashboardQuery {
            query: self.query,
            input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_DashboardQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_DashboardQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDashboardChart_DashboardQueryRef {
        ChronicleDashboardChart_DashboardQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_DashboardQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the query."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\nThe raw query string."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<ChronicleDashboardChart_DashboardQuery_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDashboardChart_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDashboardChart_Timeouts {
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

impl terrars::ToListMappable for ChronicleDashboardChart_Timeouts {
    type O = terrars::BlockAssignable<ChronicleDashboardChart_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDashboardChart_Timeouts {}

impl BuildChronicleDashboardChart_Timeouts {
    pub fn build(self) -> ChronicleDashboardChart_Timeouts {
        ChronicleDashboardChart_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDashboardChart_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDashboardChart_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDashboardChart_TimeoutsRef {
        ChronicleDashboardChart_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDashboardChart_TimeoutsRef {
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
