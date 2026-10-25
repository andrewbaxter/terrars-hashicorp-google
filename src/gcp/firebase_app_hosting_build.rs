#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirebaseAppHostingBuildData_ {
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
    backend: terrars::PrimField<std::string::String>,
    build_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<FirebaseAppHostingBuild_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirebaseAppHostingBuild_Timeouts>,
    dynamic: FirebaseAppHostingBuildDynamic,
}

struct FirebaseAppHostingBuild_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirebaseAppHostingBuildData_>,
}

#[derive(Clone)]
pub struct FirebaseAppHostingBuild(std::rc::Rc<FirebaseAppHostingBuild_>);

impl FirebaseAppHostingBuild {
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
        "Set the field `annotations`.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nHuman-readable name. 63 character limit."]
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
        "Set the field `labels`.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingBuild_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirebaseAppHostingBuild_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Build applies to"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_id` after provisioning.\nThe user-specified ID of the build being created."]
    pub fn build_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_logs_uri` after provisioning.\nThe location of the [Cloud Build\nlogs](https://cloud.google.com/build/docs/view-build-results) for the build\nprocess."]
    pub fn build_logs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_logs_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the build was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name. 63 character limit."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend when this build was created."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> terrars::ListRef<FirebaseAppHostingBuild_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_source` after provisioning.\nThe source of the error for the build, if in a 'FAILED' state.\nPossible values:\nCLOUD_BUILD\nCLOUD_RUN"]
    pub fn error_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe Artifact Registry\n[container\nimage](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages)\nURI, used by the Cloud Run\n['revision'](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions)\nfor this build."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Build applies to"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the build.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the build.\nPossible values:\nBUILDING\nBUILT\nDEPLOYING\nREADY\nFAILED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the build was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<FirebaseAppHostingBuild_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBuild_TimeoutsRef {
        FirebaseAppHostingBuild_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FirebaseAppHostingBuild {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirebaseAppHostingBuild { }

impl terrars::ToListMappable for FirebaseAppHostingBuild {
    type O = terrars::ListRef<FirebaseAppHostingBuildRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirebaseAppHostingBuild_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firebase_app_hosting_build".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirebaseAppHostingBuild {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Backend that this Build applies to"]
    pub backend: terrars::PrimField<std::string::String>,
    #[doc = "The user-specified ID of the build being created."]
    pub build_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the Backend that this Build applies to"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseAppHostingBuild {
    pub fn build(self, stack: &mut terrars::Stack) -> FirebaseAppHostingBuild {
        let out = FirebaseAppHostingBuild(std::rc::Rc::new(FirebaseAppHostingBuild_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirebaseAppHostingBuildData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                backend: self.backend,
                build_id: self.build_id,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirebaseAppHostingBuildRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuildRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirebaseAppHostingBuildRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUnstructured key value map that may be set by external tools to\nstore and arbitrary metadata. They are not queryable and should be\npreserved when modifying objects.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nThe ID of the Backend that this Build applies to"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_id` after provisioning.\nThe user-specified ID of the build being created."]
    pub fn build_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_logs_uri` after provisioning.\nThe location of the [Cloud Build\nlogs](https://cloud.google.com/build/docs/view-build-results) for the build\nprocess."]
    pub fn build_logs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_logs_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the build was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name. 63 character limit."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `environment` after provisioning.\nThe environment name of the backend when this build was created."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> terrars::ListRef<FirebaseAppHostingBuild_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_source` after provisioning.\nThe source of the error for the build, if in a 'FAILED' state.\nPossible values:\nCLOUD_BUILD\nCLOUD_RUN"]
    pub fn error_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe Artifact Registry\n[container\nimage](https://cloud.google.com/artifact-registry/docs/reference/rest/v1/projects.locations.repositories.dockerImages)\nURI, used by the Cloud Run\n['revision'](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.services.revisions)\nfor this build."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUnstructured key value map that can be used to organize and categorize\nobjects.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Backend that this Build applies to"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the build.\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/builds/{buildId}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the build.\nPossible values:\nBUILDING\nBUILT\nDEPLOYING\nREADY\nFAILED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the build was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<FirebaseAppHostingBuild_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingBuild_TimeoutsRef {
        FirebaseAppHostingBuild_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingBuildDynamic {
    source: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingBuild_Source>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingBuild_Error {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingBuild_Error {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Error {}

impl BuildFirebaseAppHostingBuild_Error {
    pub fn build(self) -> FirebaseAppHostingBuild_Error {
        FirebaseAppHostingBuild_Error {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_ErrorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingBuild_ErrorRef {
        FirebaseAppHostingBuild_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Source_Codebase_Author {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingBuild_Source_Codebase_Author {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_uri`.\n"]
    pub fn set_image_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingBuild_Source_Codebase_Author {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Source_Codebase_Author>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Source_Codebase_Author {}

impl BuildFirebaseAppHostingBuild_Source_Codebase_Author {
    pub fn build(self) -> FirebaseAppHostingBuild_Source_Codebase_Author {
        FirebaseAppHostingBuild_Source_Codebase_Author {
            display_name: core::default::Default::default(),
            email: core::default::Default::default(),
            image_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_Source_Codebase_AuthorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_Source_Codebase_AuthorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseAppHostingBuild_Source_Codebase_AuthorRef {
        FirebaseAppHostingBuild_Source_Codebase_AuthorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_Source_Codebase_AuthorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `image_uri` after provisioning.\n"]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Source_Codebase {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingBuild_Source_Codebase {
    #[doc = "Set the field `branch`.\nThe branch in the codebase to build from, using the latest commit."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit`.\nThe commit in the codebase to build from."]
    pub fn set_commit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingBuild_Source_Codebase {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Source_Codebase>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Source_Codebase {}

impl BuildFirebaseAppHostingBuild_Source_Codebase {
    pub fn build(self) -> FirebaseAppHostingBuild_Source_Codebase {
        FirebaseAppHostingBuild_Source_Codebase {
            branch: core::default::Default::default(),
            commit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_Source_CodebaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_Source_CodebaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingBuild_Source_CodebaseRef {
        FirebaseAppHostingBuild_Source_CodebaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_Source_CodebaseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nVersion control metadata for a user associated with a resolved codebase.\nCurrently assumes a Git user."]
    pub fn author(&self) -> terrars::ListRef<FirebaseAppHostingBuild_Source_Codebase_AuthorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.author", self.base))
    }

    #[doc =
        "Get a reference to the value of field `branch` after provisioning.\nThe branch in the codebase to build from, using the latest commit."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit` after provisioning.\nThe commit in the codebase to build from."]
    pub fn commit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_message` after provisioning.\nThe message of a codebase change."]
    pub fn commit_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_message", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_time` after provisioning.\nThe time the change was made."]
    pub fn commit_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-friendly name to use for this Codebase when displaying a build.\nWe use the first eight characters of the SHA-1 hash for GitHub.com."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hash` after provisioning.\nThe full SHA-1 hash of a Git commit, if available."]
    pub fn hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hash", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nA URI linking to the codebase on an hosting provider's website. May\nnot be valid if the commit has been rebased or force-pushed out of\nexistence in the linked repository."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Source_Container {
    image: terrars::PrimField<std::string::String>,
}

impl FirebaseAppHostingBuild_Source_Container { }

impl terrars::ToListMappable for FirebaseAppHostingBuild_Source_Container {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Source_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Source_Container {
    #[doc = "A URI representing a container for the backend to use."]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseAppHostingBuild_Source_Container {
    pub fn build(self) -> FirebaseAppHostingBuild_Source_Container {
        FirebaseAppHostingBuild_Source_Container { image: self.image }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_Source_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_Source_ContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingBuild_Source_ContainerRef {
        FirebaseAppHostingBuild_Source_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_Source_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nA URI representing a container for the backend to use."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingBuild_SourceDynamic {
    codebase: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingBuild_Source_Codebase>>,
    container: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingBuild_Source_Container>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codebase: std::option::Option<std::vec::Vec<FirebaseAppHostingBuild_Source_Codebase>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<FirebaseAppHostingBuild_Source_Container>>,
    dynamic: FirebaseAppHostingBuild_SourceDynamic,
}

impl FirebaseAppHostingBuild_Source {
    #[doc = "Set the field `codebase`.\n"]
    pub fn set_codebase(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingBuild_Source_Codebase>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.codebase = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.codebase = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingBuild_Source_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingBuild_Source {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Source {}

impl BuildFirebaseAppHostingBuild_Source {
    pub fn build(self) -> FirebaseAppHostingBuild_Source {
        FirebaseAppHostingBuild_Source {
            codebase: core::default::Default::default(),
            container: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingBuild_SourceRef {
        FirebaseAppHostingBuild_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `codebase` after provisioning.\n"]
    pub fn codebase(&self) -> terrars::ListRef<FirebaseAppHostingBuild_Source_CodebaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.codebase", self.base))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<FirebaseAppHostingBuild_Source_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingBuild_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingBuild_Timeouts {
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

impl terrars::ToListMappable for FirebaseAppHostingBuild_Timeouts {
    type O = terrars::BlockAssignable<FirebaseAppHostingBuild_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingBuild_Timeouts {}

impl BuildFirebaseAppHostingBuild_Timeouts {
    pub fn build(self) -> FirebaseAppHostingBuild_Timeouts {
        FirebaseAppHostingBuild_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingBuild_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingBuild_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingBuild_TimeoutsRef {
        FirebaseAppHostingBuild_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingBuild_TimeoutsRef {
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
