use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirestoreBackupScheduleData {
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
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    retention: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily_recurrence: std::option::Option<std::vec::Vec<FirestoreBackupScheduleDailyRecurrenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirestoreBackupScheduleTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_recurrence: std::option::Option<std::vec::Vec<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
    dynamic: FirestoreBackupScheduleDynamic,
}

struct FirestoreBackupSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirestoreBackupScheduleData>,
}

#[derive(Clone)]
pub struct FirestoreBackupSchedule(std::rc::Rc<FirestoreBackupSchedule_>);

impl FirestoreBackupSchedule {
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

    #[doc = "Set the field `database`.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn set_database(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `daily_recurrence`.\n"]
    pub fn set_daily_recurrence(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreBackupScheduleDailyRecurrenceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().daily_recurrence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.daily_recurrence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirestoreBackupScheduleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_recurrence`.\n"]
    pub fn set_weekly_recurrence(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().weekly_recurrence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.weekly_recurrence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique backup schedule identifier across all locations and databases for the given project. Format:\n'projects/{{project}}/databases/{{database}}/backupSchedules/{{backupSchedule}}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nFor a daily backup recurrence, set this to a value up to 7 days. If you set a weekly backup recurrence, set this to a value up to 14 weeks."]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_recurrence` after provisioning.\n"]
    pub fn daily_recurrence(&self) -> terrars::ListRef<FirestoreBackupScheduleDailyRecurrenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_recurrence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_recurrence` after provisioning.\n"]
    pub fn weekly_recurrence(&self) -> terrars::ListRef<FirestoreBackupScheduleWeeklyRecurrenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_recurrence", self.extract_ref()))
    }
}

impl terrars::Referable for FirestoreBackupSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirestoreBackupSchedule { }

impl terrars::ToListMappable for FirestoreBackupSchedule {
    type O = terrars::ListRef<FirestoreBackupScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirestoreBackupSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firestore_backup_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirestoreBackupSchedule {
    pub tf_id: std::string::String,
    #[doc =
        "At what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nFor a daily backup recurrence, set this to a value up to 7 days. If you set a weekly backup recurrence, set this to a value up to 14 weeks."]
    pub retention: terrars::PrimField<std::string::String>,
}

impl BuildFirestoreBackupSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> FirestoreBackupSchedule {
        let out = FirestoreBackupSchedule(std::rc::Rc::new(FirestoreBackupSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirestoreBackupScheduleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                retention: self.retention,
                daily_recurrence: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                weekly_recurrence: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirestoreBackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreBackupScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirestoreBackupScheduleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique backup schedule identifier across all locations and databases for the given project. Format:\n'projects/{{project}}/databases/{{database}}/backupSchedules/{{backupSchedule}}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nAt what relative time in the future, compared to its creation time, the backup should be deleted, e.g. keep backups for 7 days.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\".\n\nFor a daily backup recurrence, set this to a value up to 7 days. If you set a weekly backup recurrence, set this to a value up to 14 weeks."]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `daily_recurrence` after provisioning.\n"]
    pub fn daily_recurrence(&self) -> terrars::ListRef<FirestoreBackupScheduleDailyRecurrenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily_recurrence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `weekly_recurrence` after provisioning.\n"]
    pub fn weekly_recurrence(&self) -> terrars::ListRef<FirestoreBackupScheduleWeeklyRecurrenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_recurrence", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FirestoreBackupScheduleDailyRecurrenceEl {}

impl FirestoreBackupScheduleDailyRecurrenceEl { }

impl terrars::ToListMappable for FirestoreBackupScheduleDailyRecurrenceEl {
    type O = terrars::BlockAssignable<FirestoreBackupScheduleDailyRecurrenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreBackupScheduleDailyRecurrenceEl {}

impl BuildFirestoreBackupScheduleDailyRecurrenceEl {
    pub fn build(self) -> FirestoreBackupScheduleDailyRecurrenceEl {
        FirestoreBackupScheduleDailyRecurrenceEl {}
    }
}

pub struct FirestoreBackupScheduleDailyRecurrenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreBackupScheduleDailyRecurrenceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreBackupScheduleDailyRecurrenceElRef {
        FirestoreBackupScheduleDailyRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreBackupScheduleDailyRecurrenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct FirestoreBackupScheduleTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirestoreBackupScheduleTimeoutsEl {
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

impl terrars::ToListMappable for FirestoreBackupScheduleTimeoutsEl {
    type O = terrars::BlockAssignable<FirestoreBackupScheduleTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreBackupScheduleTimeoutsEl {}

impl BuildFirestoreBackupScheduleTimeoutsEl {
    pub fn build(self) -> FirestoreBackupScheduleTimeoutsEl {
        FirestoreBackupScheduleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FirestoreBackupScheduleTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreBackupScheduleTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreBackupScheduleTimeoutsElRef {
        FirestoreBackupScheduleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreBackupScheduleTimeoutsElRef {
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
pub struct FirestoreBackupScheduleWeeklyRecurrenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirestoreBackupScheduleWeeklyRecurrenceEl {
    #[doc =
        "Set the field `day`.\nThe day of week to run. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirestoreBackupScheduleWeeklyRecurrenceEl {
    type O = terrars::BlockAssignable<FirestoreBackupScheduleWeeklyRecurrenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreBackupScheduleWeeklyRecurrenceEl {}

impl BuildFirestoreBackupScheduleWeeklyRecurrenceEl {
    pub fn build(self) -> FirestoreBackupScheduleWeeklyRecurrenceEl {
        FirestoreBackupScheduleWeeklyRecurrenceEl { day: core::default::Default::default() }
    }
}

pub struct FirestoreBackupScheduleWeeklyRecurrenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreBackupScheduleWeeklyRecurrenceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreBackupScheduleWeeklyRecurrenceElRef {
        FirestoreBackupScheduleWeeklyRecurrenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreBackupScheduleWeeklyRecurrenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nThe day of week to run. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirestoreBackupScheduleDynamic {
    daily_recurrence: std::option::Option<terrars::DynamicBlock<FirestoreBackupScheduleDailyRecurrenceEl>>,
    weekly_recurrence: std::option::Option<terrars::DynamicBlock<FirestoreBackupScheduleWeeklyRecurrenceEl>>,
}
