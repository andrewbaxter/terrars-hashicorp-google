use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryDatasetAccessData {
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
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_by_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_member: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    special_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_by_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<std::vec::Vec<BigqueryDatasetAccessDatasetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routine: std::option::Option<std::vec::Vec<BigqueryDatasetAccessRoutineEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryDatasetAccessTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view: std::option::Option<std::vec::Vec<BigqueryDatasetAccessViewEl>>,
    dynamic: BigqueryDatasetAccessDynamic,
}

struct BigqueryDatasetAccess_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryDatasetAccessData>,
}

#[derive(Clone)]
pub struct BigqueryDatasetAccess(std::rc::Rc<BigqueryDatasetAccess_>);

impl BigqueryDatasetAccess {
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
        "Set the field `domain`.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn set_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_by_email`.\nAn email address of a Google Group to grant access to."]
    pub fn set_group_by_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group_by_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `iam_member`.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn set_iam_member(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iam_member = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `role`.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn set_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `special_group`.\nA special group to grant access to. Possible values include:\n\n\n* 'projectOwners': Owners of the enclosing project.\n\n\n* 'projectReaders': Readers of the enclosing project.\n\n\n* 'projectWriters': Writers of the enclosing project.\n\n\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn set_special_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().special_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_by_email`.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn set_user_by_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_by_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatasetAccessDatasetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routine`.\n"]
    pub fn set_routine(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatasetAccessRoutineEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routine = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routine = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryDatasetAccessTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view`.\n"]
    pub fn set_view(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatasetAccessViewEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().view = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.view = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `api_updated_member` after provisioning.\nIf true, represents that that the iam_member in the config was translated to a different member type by the API, and is stored in state as a different member type"]
    pub fn api_updated_member(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_updated_member", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_by_email` after provisioning.\nAn email address of a Google Group to grant access to."]
    pub fn group_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_member` after provisioning.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn iam_member(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_member", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `special_group` after provisioning.\nA special group to grant access to. Possible values include:\n\n\n* 'projectOwners': Owners of the enclosing project.\n\n\n* 'projectReaders': Readers of the enclosing project.\n\n\n* 'projectWriters': Writers of the enclosing project.\n\n\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn special_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.special_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_by_email` after provisioning.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn user_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_by_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::ListRef<BigqueryDatasetAccessDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> terrars::ListRef<BigqueryDatasetAccessRoutineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> terrars::ListRef<BigqueryDatasetAccessViewElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryDatasetAccess {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryDatasetAccess { }

impl terrars::ToListMappable for BigqueryDatasetAccess {
    type O = terrars::ListRef<BigqueryDatasetAccessRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryDatasetAccess_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_dataset_access".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryDatasetAccess {
    pub tf_id: std::string::String,
    #[doc =
        "A unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDatasetAccess {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryDatasetAccess {
        let out = BigqueryDatasetAccess(std::rc::Rc::new(BigqueryDatasetAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryDatasetAccessData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataset_id: self.dataset_id,
                domain: core::default::Default::default(),
                group_by_email: core::default::Default::default(),
                iam_member: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                role: core::default::Default::default(),
                special_group: core::default::Default::default(),
                user_by_email: core::default::Default::default(),
                dataset: core::default::Default::default(),
                routine: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                view: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryDatasetAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryDatasetAccessRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_updated_member` after provisioning.\nIf true, represents that that the iam_member in the config was translated to a different member type by the API, and is stored in state as a different member type"]
    pub fn api_updated_member(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_updated_member", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_by_email` after provisioning.\nAn email address of a Google Group to grant access to."]
    pub fn group_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_member` after provisioning.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn iam_member(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_member", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles are\nsupported. Predefined roles that have equivalent basic roles are\nswapped by the API to their basic counterparts, and will show a diff\npost-create. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `special_group` after provisioning.\nA special group to grant access to. Possible values include:\n\n\n* 'projectOwners': Owners of the enclosing project.\n\n\n* 'projectReaders': Readers of the enclosing project.\n\n\n* 'projectWriters': Writers of the enclosing project.\n\n\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn special_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.special_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_by_email` after provisioning.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn user_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_by_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::ListRef<BigqueryDatasetAccessDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> terrars::ListRef<BigqueryDatasetAccessRoutineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> terrars::ListRef<BigqueryDatasetAccessViewElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryDatasetAccessDatasetElDatasetEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl BigqueryDatasetAccessDatasetElDatasetEl { }

impl terrars::ToListMappable for BigqueryDatasetAccessDatasetElDatasetEl {
    type O = terrars::BlockAssignable<BigqueryDatasetAccessDatasetElDatasetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatasetAccessDatasetElDatasetEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDatasetAccessDatasetElDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessDatasetElDatasetEl {
        BigqueryDatasetAccessDatasetElDatasetEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
        }
    }
}

pub struct BigqueryDatasetAccessDatasetElDatasetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessDatasetElDatasetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatasetAccessDatasetElDatasetElRef {
        BigqueryDatasetAccessDatasetElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatasetAccessDatasetElDatasetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryDatasetAccessDatasetElDynamic {
    dataset: std::option::Option<terrars::DynamicBlock<BigqueryDatasetAccessDatasetElDatasetEl>>,
}

#[derive(Serialize)]
pub struct BigqueryDatasetAccessDatasetEl {
    target_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<std::vec::Vec<BigqueryDatasetAccessDatasetElDatasetEl>>,
    dynamic: BigqueryDatasetAccessDatasetElDynamic,
}

impl BigqueryDatasetAccessDatasetEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatasetAccessDatasetElDatasetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryDatasetAccessDatasetEl {
    type O = terrars::BlockAssignable<BigqueryDatasetAccessDatasetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatasetAccessDatasetEl {
    #[doc =
        "Which resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub target_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryDatasetAccessDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessDatasetEl {
        BigqueryDatasetAccessDatasetEl {
            target_types: self.target_types,
            dataset: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BigqueryDatasetAccessDatasetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessDatasetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatasetAccessDatasetElRef {
        BigqueryDatasetAccessDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatasetAccessDatasetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_types` after provisioning.\nWhich resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub fn target_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::ListRef<BigqueryDatasetAccessDatasetElDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDatasetAccessRoutineEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    routine_id: terrars::PrimField<std::string::String>,
}

impl BigqueryDatasetAccessRoutineEl { }

impl terrars::ToListMappable for BigqueryDatasetAccessRoutineEl {
    type O = terrars::BlockAssignable<BigqueryDatasetAccessRoutineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatasetAccessRoutineEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub routine_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDatasetAccessRoutineEl {
    pub fn build(self) -> BigqueryDatasetAccessRoutineEl {
        BigqueryDatasetAccessRoutineEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            routine_id: self.routine_id,
        }
    }
}

pub struct BigqueryDatasetAccessRoutineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessRoutineElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatasetAccessRoutineElRef {
        BigqueryDatasetAccessRoutineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatasetAccessRoutineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub fn routine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDatasetAccessTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDatasetAccessTimeoutsEl {
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
}

impl terrars::ToListMappable for BigqueryDatasetAccessTimeoutsEl {
    type O = terrars::BlockAssignable<BigqueryDatasetAccessTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatasetAccessTimeoutsEl {}

impl BuildBigqueryDatasetAccessTimeoutsEl {
    pub fn build(self) -> BigqueryDatasetAccessTimeoutsEl {
        BigqueryDatasetAccessTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct BigqueryDatasetAccessTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatasetAccessTimeoutsElRef {
        BigqueryDatasetAccessTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatasetAccessTimeoutsElRef {
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
}

#[derive(Serialize)]
pub struct BigqueryDatasetAccessViewEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryDatasetAccessViewEl { }

impl terrars::ToListMappable for BigqueryDatasetAccessViewEl {
    type O = terrars::BlockAssignable<BigqueryDatasetAccessViewEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatasetAccessViewEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDatasetAccessViewEl {
    pub fn build(self) -> BigqueryDatasetAccessViewEl {
        BigqueryDatasetAccessViewEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

pub struct BigqueryDatasetAccessViewElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatasetAccessViewElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatasetAccessViewElRef {
        BigqueryDatasetAccessViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatasetAccessViewElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryDatasetAccessDynamic {
    dataset: std::option::Option<terrars::DynamicBlock<BigqueryDatasetAccessDatasetEl>>,
    routine: std::option::Option<terrars::DynamicBlock<BigqueryDatasetAccessRoutineEl>>,
    view: std::option::Option<terrars::DynamicBlock<BigqueryDatasetAccessViewEl>>,
}
