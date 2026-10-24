use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexTaskData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lake: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_spec: std::option::Option<std::vec::Vec<DataplexTaskExecutionSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook: std::option::Option<std::vec::Vec<DataplexTaskNotebookEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark: std::option::Option<std::vec::Vec<DataplexTaskSparkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexTaskTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_spec: std::option::Option<std::vec::Vec<DataplexTaskTriggerSpecEl>>,
    dynamic: DataplexTaskDynamic,
}

struct DataplexTask_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexTaskData>,
}

#[derive(Clone)]
pub struct DataplexTask(std::rc::Rc<DataplexTask_>);

impl DataplexTask {
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

    #[doc = "Set the field `description`.\nUser-provided description of the task."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser friendly display name."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lake`.\nThe lake in which the task will be created in."]
    pub fn set_lake(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lake = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location in which the task will be created in."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_id`.\nThe task Id of the task."]
    pub fn set_task_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().task_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_spec`.\n"]
    pub fn set_execution_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskExecutionSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notebook`.\n"]
    pub fn set_notebook(self, v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskNotebookEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notebook = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notebook = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(self, v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskSparkEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexTaskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_spec`.\n"]
    pub fn set_trigger_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskTriggerSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the task was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the task."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_status` after provisioning.\nConfiguration for the cluster"]
    pub fn execution_status(&self) -> terrars::ListRef<DataplexTaskExecutionStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lake` after provisioning.\nThe lake in which the task will be created in."]
    pub fn lake(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lake", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the task will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the task, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/ tasks/{name}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the task."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_id` after provisioning.\nThe task Id of the task."]
    pub fn task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the task was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> terrars::ListRef<DataplexTaskExecutionSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notebook` after provisioning.\n"]
    pub fn notebook(&self) -> terrars::ListRef<DataplexTaskNotebookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notebook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<DataplexTaskSparkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_spec` after provisioning.\n"]
    pub fn trigger_spec(&self) -> terrars::ListRef<DataplexTaskTriggerSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_spec", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexTask {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexTask { }

impl terrars::ToListMappable for DataplexTask {
    type O = terrars::ListRef<DataplexTaskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexTask_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_task".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexTask {
    pub tf_id: std::string::String,
}

impl BuildDataplexTask {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexTask {
        let out = DataplexTask(std::rc::Rc::new(DataplexTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexTaskData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lake: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                task_id: core::default::Default::default(),
                execution_spec: core::default::Default::default(),
                notebook: core::default::Default::default(),
                spark: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                trigger_spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataplexTaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexTaskRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the task was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the task."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_status` after provisioning.\nConfiguration for the cluster"]
    pub fn execution_status(&self) -> terrars::ListRef<DataplexTaskExecutionStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the task.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lake` after provisioning.\nThe lake in which the task will be created in."]
    pub fn lake(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lake", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the task will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the task, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/ tasks/{name}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the task."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_id` after provisioning.\nThe task Id of the task."]
    pub fn task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated globally unique ID for the task. This ID will be different if the task is deleted and re-created with the same name."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the task was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_spec` after provisioning.\n"]
    pub fn execution_spec(&self) -> terrars::ListRef<DataplexTaskExecutionSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notebook` after provisioning.\n"]
    pub fn notebook(&self) -> terrars::ListRef<DataplexTaskNotebookElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notebook", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<DataplexTaskSparkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_spec` after provisioning.\n"]
    pub fn trigger_spec(&self) -> terrars::ListRef<DataplexTaskTriggerSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskExecutionStatusElLatestJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_job: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexTaskExecutionStatusElLatestJobEl {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_count`.\n"]
    pub fn set_retry_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_job`.\n"]
    pub fn set_service_job(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskExecutionStatusElLatestJobEl {
    type O = terrars::BlockAssignable<DataplexTaskExecutionStatusElLatestJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskExecutionStatusElLatestJobEl {}

impl BuildDataplexTaskExecutionStatusElLatestJobEl {
    pub fn build(self) -> DataplexTaskExecutionStatusElLatestJobEl {
        DataplexTaskExecutionStatusElLatestJobEl {
            end_time: core::default::Default::default(),
            message: core::default::Default::default(),
            name: core::default::Default::default(),
            retry_count: core::default::Default::default(),
            service: core::default::Default::default(),
            service_job: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskExecutionStatusElLatestJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskExecutionStatusElLatestJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskExecutionStatusElLatestJobElRef {
        DataplexTaskExecutionStatusElLatestJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskExecutionStatusElLatestJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_count` after provisioning.\n"]
    pub fn retry_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_count", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `service_job` after provisioning.\n"]
    pub fn service_job(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_job", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskExecutionStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_job: std::option::Option<terrars::ListField<DataplexTaskExecutionStatusElLatestJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexTaskExecutionStatusEl {
    #[doc = "Set the field `latest_job`.\n"]
    pub fn set_latest_job(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataplexTaskExecutionStatusElLatestJobEl>>,
    ) -> Self {
        self.latest_job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskExecutionStatusEl {
    type O = terrars::BlockAssignable<DataplexTaskExecutionStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskExecutionStatusEl {}

impl BuildDataplexTaskExecutionStatusEl {
    pub fn build(self) -> DataplexTaskExecutionStatusEl {
        DataplexTaskExecutionStatusEl {
            latest_job: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskExecutionStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskExecutionStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskExecutionStatusElRef {
        DataplexTaskExecutionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskExecutionStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latest_job` after provisioning.\n"]
    pub fn latest_job(&self) -> terrars::ListRef<DataplexTaskExecutionStatusElLatestJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latest_job", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskExecutionSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_job_execution_lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account: terrars::PrimField<std::string::String>,
}

impl DataplexTaskExecutionSpecEl {
    #[doc =
        "Set the field `args`.\nThe arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${taskId} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument. An object containing a list of 'key': value pairs. Example: { 'name': 'wrench', 'mass': '1.3kg', 'count': '3' }."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{locationId}/keyRings/{key-ring-name}/cryptoKeys/{key-name}."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_job_execution_lifetime`.\nThe maximum duration after which the job execution is expired. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn set_max_job_execution_lifetime(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_job_execution_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which jobs are run. By default, the project containing the Lake is used. If a project is provided, the ExecutionSpec.service_account must belong to this project."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskExecutionSpecEl {
    type O = terrars::BlockAssignable<DataplexTaskExecutionSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskExecutionSpecEl {
    #[doc =
        "Service account to use to execute a task. If not provided, the default Compute service account for the project is used."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildDataplexTaskExecutionSpecEl {
    pub fn build(self) -> DataplexTaskExecutionSpecEl {
        DataplexTaskExecutionSpecEl {
            args: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            max_job_execution_lifetime: core::default::Default::default(),
            project: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}

pub struct DataplexTaskExecutionSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskExecutionSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskExecutionSpecElRef {
        DataplexTaskExecutionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskExecutionSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the task. The args can use placeholders of the format ${placeholder} as part of key/value string. These will be interpolated before passing the args to the driver. Currently supported placeholders: - ${taskId} - ${job_time} To pass positional args, set the key as TASK_ARGS. The value should be a comma-separated string of all the positional arguments. To use a delimiter other than comma, refer to https://cloud.google.com/sdk/gcloud/reference/topic/escaping. In case of other keys being present in the args, then TASK_ARGS will be passed as the last argument. An object containing a list of 'key': value pairs. Example: { 'name': 'wrench', 'mass': '1.3kg', 'count': '3' }."]
    pub fn args(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption, of the form: projects/{project_number}/locations/{locationId}/keyRings/{key-ring-name}/cryptoKeys/{key-name}."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_job_execution_lifetime` after provisioning.\nThe maximum duration after which the job execution is expired. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn max_job_execution_lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_job_execution_lifetime", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which jobs are run. By default, the project containing the Lake is used. If a project is provided, the ExecutionSpec.service_account must belong to this project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account to use to execute a task. If not provided, the default Compute service account for the project is used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executors_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_executors_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    #[doc =
        "Set the field `executors_count`.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn set_executors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.executors_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_executors_count`.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn set_max_executors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_executors_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskNotebookElInfrastructureSpecElBatchEl {
    type O = terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElBatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskNotebookElInfrastructureSpecElBatchEl {}

impl BuildDataplexTaskNotebookElInfrastructureSpecElBatchEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElBatchEl {
        DataplexTaskNotebookElInfrastructureSpecElBatchEl {
            executors_count: core::default::Default::default(),
            max_executors_count: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
        DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskNotebookElInfrastructureSpecElBatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `executors_count` after provisioning.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn executors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.executors_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_executors_count` after provisioning.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn max_executors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_executors_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    java_jars: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_packages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    #[doc = "Set the field `image`.\nContainer image to use."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `java_jars`.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn set_java_jars(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.java_jars = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_packages`.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn set_python_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_packages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    type O = terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskNotebookElInfrastructureSpecElContainerImageEl {}

impl BuildDataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
        DataplexTaskNotebookElInfrastructureSpecElContainerImageEl {
            image: core::default::Default::default(),
            java_jars: core::default::Default::default(),
            properties: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
        DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nContainer image to use."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `java_jars` after provisioning.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn java_jars(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.java_jars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_packages` after provisioning.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn python_packages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_packages", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    #[doc =
        "Set the field `network`.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\nList of network tags to apply to the job."]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_network`.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn set_sub_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    type O = terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {}

impl BuildDataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
        DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl {
            network: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            sub_network: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
        DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nList of network tags to apply to the job."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sub_network` after provisioning.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn sub_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexTaskNotebookElInfrastructureSpecElDynamic {
    batch: std::option::Option<terrars::DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    container_image: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    vpc_network: std::option::Option<terrars::DynamicBlock<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
}

#[derive(Serialize)]
pub struct DataplexTaskNotebookElInfrastructureSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch: std::option::Option<std::vec::Vec<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<std::vec::Vec<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_network: std::option::Option<std::vec::Vec<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
    dynamic: DataplexTaskNotebookElInfrastructureSpecElDynamic,
}

impl DataplexTaskNotebookElInfrastructureSpecEl {
    #[doc = "Set the field `batch`.\n"]
    pub fn set_batch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElBatchEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElContainerImageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_network`.\n"]
    pub fn set_vpc_network(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexTaskNotebookElInfrastructureSpecEl {
    type O = terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskNotebookElInfrastructureSpecEl {}

impl BuildDataplexTaskNotebookElInfrastructureSpecEl {
    pub fn build(self) -> DataplexTaskNotebookElInfrastructureSpecEl {
        DataplexTaskNotebookElInfrastructureSpecEl {
            batch: core::default::Default::default(),
            container_image: core::default::Default::default(),
            vpc_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskNotebookElInfrastructureSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskNotebookElInfrastructureSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskNotebookElInfrastructureSpecElRef {
        DataplexTaskNotebookElInfrastructureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskNotebookElInfrastructureSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch` after provisioning.\n"]
    pub fn batch(&self) -> terrars::ListRef<DataplexTaskNotebookElInfrastructureSpecElBatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch", self.base))
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> terrars::ListRef<DataplexTaskNotebookElInfrastructureSpecElContainerImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_network` after provisioning.\n"]
    pub fn vpc_network(&self) -> terrars::ListRef<DataplexTaskNotebookElInfrastructureSpecElVpcNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexTaskNotebookElDynamic {
    infrastructure_spec: std::option::Option<terrars::DynamicBlock<DataplexTaskNotebookElInfrastructureSpecEl>>,
}

#[derive(Serialize)]
pub struct DataplexTaskNotebookEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    notebook: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infrastructure_spec: std::option::Option<std::vec::Vec<DataplexTaskNotebookElInfrastructureSpecEl>>,
    dynamic: DataplexTaskNotebookElDynamic,
}

impl DataplexTaskNotebookEl {
    #[doc =
        "Set the field `archive_uris`.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infrastructure_spec`.\n"]
    pub fn set_infrastructure_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskNotebookElInfrastructureSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infrastructure_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infrastructure_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexTaskNotebookEl {
    type O = terrars::BlockAssignable<DataplexTaskNotebookEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskNotebookEl {
    #[doc =
        "Path to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value)."]
    pub notebook: terrars::PrimField<std::string::String>,
}

impl BuildDataplexTaskNotebookEl {
    pub fn build(self) -> DataplexTaskNotebookEl {
        DataplexTaskNotebookEl {
            archive_uris: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            notebook: self.notebook,
            infrastructure_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskNotebookElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskNotebookElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskNotebookElRef {
        DataplexTaskNotebookElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskNotebookElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notebook` after provisioning.\nPath to input notebook. This can be the Cloud Storage URI of the notebook file or the path to a Notebook Content. The execution args are accessible as environment variables (TASK_key=value)."]
    pub fn notebook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook", self.base))
    }

    #[doc = "Get a reference to the value of field `infrastructure_spec` after provisioning.\n"]
    pub fn infrastructure_spec(&self) -> terrars::ListRef<DataplexTaskNotebookElInfrastructureSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infrastructure_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElBatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executors_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_executors_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataplexTaskSparkElInfrastructureSpecElBatchEl {
    #[doc =
        "Set the field `executors_count`.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn set_executors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.executors_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_executors_count`.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn set_max_executors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_executors_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskSparkElInfrastructureSpecElBatchEl {
    type O = terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElBatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskSparkElInfrastructureSpecElBatchEl {}

impl BuildDataplexTaskSparkElInfrastructureSpecElBatchEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElBatchEl {
        DataplexTaskSparkElInfrastructureSpecElBatchEl {
            executors_count: core::default::Default::default(),
            max_executors_count: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskSparkElInfrastructureSpecElBatchElRef {
        DataplexTaskSparkElInfrastructureSpecElBatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskSparkElInfrastructureSpecElBatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `executors_count` after provisioning.\nTotal number of job executors. Executor Count should be between 2 and 100. [Default=2]"]
    pub fn executors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.executors_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_executors_count` after provisioning.\nMax configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]"]
    pub fn max_executors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_executors_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    java_jars: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_packages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    #[doc = "Set the field `image`.\nContainer image to use."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `java_jars`.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn set_java_jars(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.java_jars = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_packages`.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn set_python_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_packages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    type O = terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskSparkElInfrastructureSpecElContainerImageEl {}

impl BuildDataplexTaskSparkElInfrastructureSpecElContainerImageEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
        DataplexTaskSparkElInfrastructureSpecElContainerImageEl {
            image: core::default::Default::default(),
            java_jars: core::default::Default::default(),
            properties: core::default::Default::default(),
            python_packages: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
        DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskSparkElInfrastructureSpecElContainerImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nContainer image to use."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `java_jars` after provisioning.\nA list of Java JARS to add to the classpath. Valid input includes Cloud Storage URIs to Jar binaries. For example, gs://bucket-name/my/path/to/file.jar"]
    pub fn java_jars(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.java_jars", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOverride to common configuration of open source components installed on the Dataproc cluster. The properties to set on daemon config files. Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. For more information, see Cluster properties."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_packages` after provisioning.\nA list of python packages to be installed. Valid formats include Cloud Storage URI to a PIP installable library. For example, gs://bucket-name/my/path/to/lib.tar.gz"]
    pub fn python_packages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_packages", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_network: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    #[doc =
        "Set the field `network`.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\nList of network tags to apply to the job."]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_network`.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn set_sub_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_network = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    type O = terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {}

impl BuildDataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
        DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl {
            network: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            sub_network: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
        DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nList of network tags to apply to the job."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sub_network` after provisioning.\nThe Cloud VPC sub-network in which the job is run."]
    pub fn sub_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexTaskSparkElInfrastructureSpecElDynamic {
    batch: std::option::Option<terrars::DynamicBlock<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    container_image: std
    ::option
    ::Option<terrars::DynamicBlock<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    vpc_network: std::option::Option<terrars::DynamicBlock<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
}

#[derive(Serialize)]
pub struct DataplexTaskSparkElInfrastructureSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch: std::option::Option<std::vec::Vec<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<std::vec::Vec<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_network: std::option::Option<std::vec::Vec<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
    dynamic: DataplexTaskSparkElInfrastructureSpecElDynamic,
}

impl DataplexTaskSparkElInfrastructureSpecEl {
    #[doc = "Set the field `batch`.\n"]
    pub fn set_batch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElBatchEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElContainerImageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_network`.\n"]
    pub fn set_vpc_network(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecElVpcNetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexTaskSparkElInfrastructureSpecEl {
    type O = terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskSparkElInfrastructureSpecEl {}

impl BuildDataplexTaskSparkElInfrastructureSpecEl {
    pub fn build(self) -> DataplexTaskSparkElInfrastructureSpecEl {
        DataplexTaskSparkElInfrastructureSpecEl {
            batch: core::default::Default::default(),
            container_image: core::default::Default::default(),
            vpc_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskSparkElInfrastructureSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskSparkElInfrastructureSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskSparkElInfrastructureSpecElRef {
        DataplexTaskSparkElInfrastructureSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskSparkElInfrastructureSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch` after provisioning.\n"]
    pub fn batch(&self) -> terrars::ListRef<DataplexTaskSparkElInfrastructureSpecElBatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch", self.base))
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> terrars::ListRef<DataplexTaskSparkElInfrastructureSpecElContainerImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_network` after provisioning.\n"]
    pub fn vpc_network(&self) -> terrars::ListRef<DataplexTaskSparkElInfrastructureSpecElVpcNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexTaskSparkElDynamic {
    infrastructure_spec: std::option::Option<terrars::DynamicBlock<DataplexTaskSparkElInfrastructureSpecEl>>,
}

#[derive(Serialize)]
pub struct DataplexTaskSparkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_jar_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_script_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_script_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infrastructure_spec: std::option::Option<std::vec::Vec<DataplexTaskSparkElInfrastructureSpecEl>>,
    dynamic: DataplexTaskSparkElDynamic,
}

impl DataplexTaskSparkEl {
    #[doc =
        "Set the field `archive_uris`.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_jar_file_uri`.\nThe Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_script_file`.\nThe Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn set_python_script_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.python_script_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sql_script`.\nThe query text. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn set_sql_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_script = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sql_script_file`.\nA reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn set_sql_script_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_script_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infrastructure_spec`.\n"]
    pub fn set_infrastructure_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexTaskSparkElInfrastructureSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infrastructure_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infrastructure_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexTaskSparkEl {
    type O = terrars::BlockAssignable<DataplexTaskSparkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskSparkEl {}

impl BuildDataplexTaskSparkEl {
    pub fn build(self) -> DataplexTaskSparkEl {
        DataplexTaskSparkEl {
            archive_uris: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            python_script_file: core::default::Default::default(),
            sql_script: core::default::Default::default(),
            sql_script_file: core::default::Default::default(),
            infrastructure_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskSparkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskSparkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskSparkElRef {
        DataplexTaskSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskSparkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nCloud Storage URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nCloud Storage URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe Cloud Storage URI of the jar file that contains the main class. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_script_file` after provisioning.\nThe Gcloud Storage URI of the main Python file to use as the driver. Must be a .py file. The execution args are passed in as a sequence of named process arguments (--key=value)."]
    pub fn python_script_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_script_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sql_script` after provisioning.\nThe query text. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn sql_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sql_script_file` after provisioning.\nA reference to a query file. This can be the Cloud Storage URI of the query file or it can the path to a SqlScript Content. The execution args are used to declare a set of script variables (set key='value';)."]
    pub fn sql_script_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_script_file", self.base))
    }

    #[doc = "Get a reference to the value of field `infrastructure_spec` after provisioning.\n"]
    pub fn infrastructure_spec(&self) -> terrars::ListRef<DataplexTaskSparkElInfrastructureSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infrastructure_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexTaskTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexTaskTimeoutsEl {
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

impl terrars::ToListMappable for DataplexTaskTimeoutsEl {
    type O = terrars::BlockAssignable<DataplexTaskTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskTimeoutsEl {}

impl BuildDataplexTaskTimeoutsEl {
    pub fn build(self) -> DataplexTaskTimeoutsEl {
        DataplexTaskTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataplexTaskTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskTimeoutsElRef {
        DataplexTaskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskTimeoutsElRef {
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
pub struct DataplexTaskTriggerSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataplexTaskTriggerSpecEl {
    #[doc =
        "Set the field `disabled`.\nPrevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retries`.\nNumber of retry attempts before aborting. Set to zero to never attempt to retry a failed task."]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nCron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: 'CRON_TZ=${IANA_TIME_ZONE}' or 'TZ=${IANA_TIME_ZONE}'. The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks."]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nThe first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexTaskTriggerSpecEl {
    type O = terrars::BlockAssignable<DataplexTaskTriggerSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexTaskTriggerSpecEl {
    #[doc = "Trigger type of the user-specified Task Possible values: [\"ON_DEMAND\", \"RECURRING\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataplexTaskTriggerSpecEl {
    pub fn build(self) -> DataplexTaskTriggerSpecEl {
        DataplexTaskTriggerSpecEl {
            disabled: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct DataplexTaskTriggerSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexTaskTriggerSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexTaskTriggerSpecElRef {
        DataplexTaskTriggerSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexTaskTriggerSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nPrevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retries` after provisioning.\nNumber of retry attempts before aborting. Set to zero to never attempt to retry a failed task."]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: 'CRON_TZ=${IANA_TIME_ZONE}' or 'TZ=${IANA_TIME_ZONE}'. The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nTrigger type of the user-specified Task Possible values: [\"ON_DEMAND\", \"RECURRING\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexTaskDynamic {
    execution_spec: std::option::Option<terrars::DynamicBlock<DataplexTaskExecutionSpecEl>>,
    notebook: std::option::Option<terrars::DynamicBlock<DataplexTaskNotebookEl>>,
    spark: std::option::Option<terrars::DynamicBlock<DataplexTaskSparkEl>>,
    trigger_spec: std::option::Option<terrars::DynamicBlock<DataplexTaskTriggerSpecEl>>,
}
