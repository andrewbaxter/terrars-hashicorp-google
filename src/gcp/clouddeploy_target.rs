use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployTargetData {
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
    deploy_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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
    require_approval: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthos_cluster: std::option::Option<std::vec::Vec<ClouddeployTargetAnthosClusterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_configs: std::option::Option<std::vec::Vec<ClouddeployTargetExecutionConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke: std::option::Option<std::vec::Vec<ClouddeployTargetGkeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_target: std::option::Option<std::vec::Vec<ClouddeployTargetMultiTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run: std::option::Option<std::vec::Vec<ClouddeployTargetRunEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployTargetTimeoutsEl>,
    dynamic: ClouddeployTargetDynamic,
}

struct ClouddeployTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployTargetData>,
}

#[derive(Clone)]
pub struct ClouddeployTarget(std::rc::Rc<ClouddeployTarget_>);

impl ClouddeployTarget {
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
        "Set the field `annotations`.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_parameters`.\nOptional. The deploy parameters to use for this target."]
    pub fn set_deploy_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().deploy_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Set the field `labels`.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_approval`.\nOptional. Whether or not the `Target` requires approval."]
    pub fn set_require_approval(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_approval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthos_cluster`.\n"]
    pub fn set_anthos_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTargetAnthosClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anthos_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anthos_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_configs`.\n"]
    pub fn set_execution_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTargetExecutionConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gke`.\n"]
    pub fn set_gke(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTargetGkeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_target`.\n"]
    pub fn set_multi_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTargetMultiTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run`.\n"]
    pub fn set_run(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTargetRunEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployTargetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is [a-z][a-z0-9\\-]{0,62}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> terrars::ListRef<ClouddeployTargetAnthosClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthos_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> terrars::ListRef<ClouddeployTargetExecutionConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> terrars::ListRef<ClouddeployTargetGkeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> terrars::ListRef<ClouddeployTargetMultiTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> terrars::ListRef<ClouddeployTargetRunElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ClouddeployTarget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployTarget { }

impl terrars::ToListMappable for ClouddeployTarget {
    type O = terrars::ListRef<ClouddeployTargetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployTarget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_target".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployTarget {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the `Target`. Format is [a-z][a-z0-9\\-]{0,62}."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployTarget {
        let out = ClouddeployTarget(std::rc::Rc::new(ClouddeployTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployTargetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deploy_parameters: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                require_approval: core::default::Default::default(),
                anthos_cluster: core::default::Default::default(),
                execution_configs: core::default::Default::default(),
                gke: core::default::Default::default(),
                multi_target: core::default::Default::default(),
                run: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployTargetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is [a-z][a-z0-9\\-]{0,62}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> terrars::ListRef<ClouddeployTargetAnthosClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthos_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> terrars::ListRef<ClouddeployTargetExecutionConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> terrars::ListRef<ClouddeployTargetGkeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> terrars::ListRef<ClouddeployTargetMultiTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> terrars::ListRef<ClouddeployTargetRunElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetAnthosClusterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTargetAnthosClusterEl {
    #[doc =
        "Set the field `membership`.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTargetAnthosClusterEl {
    type O = terrars::BlockAssignable<ClouddeployTargetAnthosClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetAnthosClusterEl {}

impl BuildClouddeployTargetAnthosClusterEl {
    pub fn build(self) -> ClouddeployTargetAnthosClusterEl {
        ClouddeployTargetAnthosClusterEl { membership: core::default::Default::default() }
    }
}

pub struct ClouddeployTargetAnthosClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetAnthosClusterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetAnthosClusterElRef {
        ClouddeployTargetAnthosClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetAnthosClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetExecutionConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    usages: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTargetExecutionConfigsEl {
    #[doc =
        "Set the field `artifact_storage`.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_timeout`.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_pool`.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTargetExecutionConfigsEl {
    type O = terrars::BlockAssignable<ClouddeployTargetExecutionConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetExecutionConfigsEl {
    #[doc = "Required. Usages when this configuration should be applied."]
    pub usages: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployTargetExecutionConfigsEl {
    pub fn build(self) -> ClouddeployTargetExecutionConfigsEl {
        ClouddeployTargetExecutionConfigsEl {
            artifact_storage: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
            service_account: core::default::Default::default(),
            usages: self.usages,
            worker_pool: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployTargetExecutionConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetExecutionConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetExecutionConfigsElRef {
        ClouddeployTargetExecutionConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetExecutionConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usages` after provisioning.\nRequired. Usages when this configuration should be applied."]
    pub fn usages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetGkeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployTargetGkeEl {
    #[doc =
        "Set the field `cluster`.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `internal_ip`.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTargetGkeEl {
    type O = terrars::BlockAssignable<ClouddeployTargetGkeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetGkeEl {}

impl BuildClouddeployTargetGkeEl {
    pub fn build(self) -> ClouddeployTargetGkeEl {
        ClouddeployTargetGkeEl {
            cluster: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployTargetGkeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetGkeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetGkeElRef {
        ClouddeployTargetGkeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetGkeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `internal_ip` after provisioning.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn internal_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetMultiTargetEl {
    target_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTargetMultiTargetEl { }

impl terrars::ToListMappable for ClouddeployTargetMultiTargetEl {
    type O = terrars::BlockAssignable<ClouddeployTargetMultiTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetMultiTargetEl {
    #[doc = "Required. The target_ids of this multiTarget."]
    pub target_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployTargetMultiTargetEl {
    pub fn build(self) -> ClouddeployTargetMultiTargetEl {
        ClouddeployTargetMultiTargetEl { target_ids: self.target_ids }
    }
}

pub struct ClouddeployTargetMultiTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetMultiTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetMultiTargetElRef {
        ClouddeployTargetMultiTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetMultiTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_ids` after provisioning.\nRequired. The target_ids of this multiTarget."]
    pub fn target_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetRunEl {
    location: terrars::PrimField<std::string::String>,
}

impl ClouddeployTargetRunEl { }

impl terrars::ToListMappable for ClouddeployTargetRunEl {
    type O = terrars::BlockAssignable<ClouddeployTargetRunEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetRunEl {
    #[doc =
        "Required. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTargetRunEl {
    pub fn build(self) -> ClouddeployTargetRunEl {
        ClouddeployTargetRunEl { location: self.location }
    }
}

pub struct ClouddeployTargetRunElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetRunElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetRunElRef {
        ClouddeployTargetRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetRunElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nRequired. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTargetTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTargetTimeoutsEl {
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

impl terrars::ToListMappable for ClouddeployTargetTimeoutsEl {
    type O = terrars::BlockAssignable<ClouddeployTargetTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTargetTimeoutsEl {}

impl BuildClouddeployTargetTimeoutsEl {
    pub fn build(self) -> ClouddeployTargetTimeoutsEl {
        ClouddeployTargetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ClouddeployTargetTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTargetTimeoutsElRef {
        ClouddeployTargetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTargetTimeoutsElRef {
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
struct ClouddeployTargetDynamic {
    anthos_cluster: std::option::Option<terrars::DynamicBlock<ClouddeployTargetAnthosClusterEl>>,
    execution_configs: std::option::Option<terrars::DynamicBlock<ClouddeployTargetExecutionConfigsEl>>,
    gke: std::option::Option<terrars::DynamicBlock<ClouddeployTargetGkeEl>>,
    multi_target: std::option::Option<terrars::DynamicBlock<ClouddeployTargetMultiTargetEl>>,
    run: std::option::Option<terrars::DynamicBlock<ClouddeployTargetRunEl>>,
}
