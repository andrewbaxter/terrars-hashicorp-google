#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MonitoringAlertPolicyData_ {
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
    combiner: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_channels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert_strategy: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_AlertStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Documentation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MonitoringAlertPolicy_Timeouts>,
    dynamic: MonitoringAlertPolicyDynamic,
}

struct MonitoringAlertPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MonitoringAlertPolicyData_>,
}

#[derive(Clone)]
pub struct MonitoringAlertPolicy(std::rc::Rc<MonitoringAlertPolicy_>);

impl MonitoringAlertPolicy {
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

    #[doc = "Set the field `enabled`.\nWhether or not the policy is enabled. The default is true."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_channels`.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn set_notification_channels(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().notification_channels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `severity`.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn set_severity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().severity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_labels`.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn set_user_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert_strategy`.\n"]
    pub fn set_alert_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alert_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alert_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `documentation`.\n"]
    pub fn set_documentation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Documentation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().documentation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.documentation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MonitoringAlertPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `combiner` after provisioning.\nHow to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub fn combiner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.combiner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_record` after provisioning.\nA read-only record of the creation of the alerting policy.\nIf provided in a call to create or update, this field will\nbe ignored."]
    pub fn creation_record(&self) -> terrars::ListRef<MonitoringAlertPolicy_CreationRecordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_record", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the policy is enabled. The default is true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this policy.\nIts syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_channels` after provisioning.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alert_strategy` after provisioning.\n"]
    pub fn alert_strategy(&self) -> terrars::ListRef<MonitoringAlertPolicy_AlertStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<MonitoringAlertPolicy_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> terrars::ListRef<MonitoringAlertPolicy_DocumentationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringAlertPolicy_TimeoutsRef {
        MonitoringAlertPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MonitoringAlertPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MonitoringAlertPolicy { }

impl terrars::ToListMappable for MonitoringAlertPolicy {
    type O = terrars::ListRef<MonitoringAlertPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MonitoringAlertPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_monitoring_alert_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitoringAlertPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "How to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub combiner: terrars::PrimField<std::string::String>,
    #[doc =
        "A short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> MonitoringAlertPolicy {
        let out = MonitoringAlertPolicy(std::rc::Rc::new(MonitoringAlertPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MonitoringAlertPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                combiner: self.combiner,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                notification_channels: core::default::Default::default(),
                project: core::default::Default::default(),
                severity: core::default::Default::default(),
                user_labels: core::default::Default::default(),
                alert_strategy: core::default::Default::default(),
                conditions: core::default::Default::default(),
                documentation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitoringAlertPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitoringAlertPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `combiner` after provisioning.\nHow to combine the results of multiple conditions to\ndetermine if an incident should be opened. Possible values: [\"AND\", \"OR\", \"AND_WITH_MATCHING_RESOURCE\"]"]
    pub fn combiner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.combiner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_record` after provisioning.\nA read-only record of the creation of the alerting policy.\nIf provided in a call to create or update, this field will\nbe ignored."]
    pub fn creation_record(&self) -> terrars::ListRef<MonitoringAlertPolicy_CreationRecordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_record", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the policy in\ndashboards, notifications, and incidents. To avoid confusion, don't use\nthe same display name for multiple policies in the same project. The\nname is limited to 512 Unicode characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether or not the policy is enabled. The default is true."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this policy.\nIts syntax is: projects/[PROJECT_ID]/alertPolicies/[ALERT_POLICY_ID]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_channels` after provisioning.\nIdentifies the notification channels to which notifications should be\nsent when incidents are opened or closed or when new violations occur\non an already opened incident. Each element of this array corresponds\nto the name field in each of the NotificationChannel objects that are\nreturned from the notificationChannels.list method. The syntax of the\nentries in this field is\n'projects/[PROJECT_ID]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nThe severity of an alert policy indicates how important incidents generated\nby that policy are. The severity level will be displayed on the Incident\ndetail page and in notifications. Possible values: [\"CRITICAL\", \"ERROR\", \"WARNING\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alert_strategy` after provisioning.\n"]
    pub fn alert_strategy(&self) -> terrars::ListRef<MonitoringAlertPolicy_AlertStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<MonitoringAlertPolicy_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> terrars::ListRef<MonitoringAlertPolicy_DocumentationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringAlertPolicy_TimeoutsRef {
        MonitoringAlertPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicyDynamic {
    alert_strategy: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_AlertStrategy>>,
    conditions: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions>>,
    documentation: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Documentation>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_CreationRecord {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutate_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutated_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_CreationRecord {
    #[doc = "Set the field `mutate_time`.\n"]
    pub fn set_mutate_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mutate_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mutated_by`.\n"]
    pub fn set_mutated_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mutated_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_CreationRecord {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_CreationRecord>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_CreationRecord {}

impl BuildMonitoringAlertPolicy_CreationRecord {
    pub fn build(self) -> MonitoringAlertPolicy_CreationRecord {
        MonitoringAlertPolicy_CreationRecord {
            mutate_time: core::default::Default::default(),
            mutated_by: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_CreationRecordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_CreationRecordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_CreationRecordRef {
        MonitoringAlertPolicy_CreationRecordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_CreationRecordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mutate_time` after provisioning.\n"]
    pub fn mutate_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutate_time", self.base))
    }

    #[doc = "Get a reference to the value of field `mutated_by` after provisioning.\n"]
    pub fn mutated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutated_by", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_channel_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renotify_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
    #[doc =
        "Set the field `notification_channel_names`.\nThe notification channels that these settings apply to. Each of these\ncorrespond to the name field in one of the NotificationChannel objects\nreferenced in the notification_channels field of this AlertPolicy. The format is\n'projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]'"]
    pub fn set_notification_channel_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.notification_channel_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `renotify_interval`.\nThe frequency at which to send reminder notifications for open incidents."]
    pub fn set_renotify_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.renotify_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {}

impl BuildMonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
    pub fn build(self) -> MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
        MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy {
            notification_channel_names: core::default::Default::default(),
            renotify_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef {
        MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `notification_channel_names` after provisioning.\nThe notification channels that these settings apply to. Each of these\ncorrespond to the name field in one of the NotificationChannel objects\nreferenced in the notification_channels field of this AlertPolicy. The format is\n'projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]'"]
    pub fn notification_channel_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channel_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `renotify_interval` after provisioning.\nThe frequency at which to send reminder notifications for open incidents."]
    pub fn renotify_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renotify_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {
    #[doc =
        "Set the field `period`.\nNot more than one notification per period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"60.5s\"."]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {}

impl BuildMonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {
    pub fn build(self) -> MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit {
        MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit { period: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef {
        MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nNot more than one notification per period.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"60.5s\"."]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_AlertStrategyDynamic {
    notification_channel_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy>>,
    notification_rate_limit: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_AlertStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_close: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_prompts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_channel_strategy: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_rate_limit: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit>>,
    dynamic: MonitoringAlertPolicy_AlertStrategyDynamic,
}

impl MonitoringAlertPolicy_AlertStrategy {
    #[doc =
        "Set the field `auto_close`.\nIf an alert policy that was active has no data for this long, any open incidents will close."]
    pub fn set_auto_close(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_close = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_prompts`.\nControl when notifications will be sent out. Possible values: [\"NOTIFICATION_PROMPT_UNSPECIFIED\", \"OPENED\", \"CLOSED\"]"]
    pub fn set_notification_prompts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.notification_prompts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_channel_strategy`.\n"]
    pub fn set_notification_channel_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_channel_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_channel_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_rate_limit`.\n"]
    pub fn set_notification_rate_limit(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy_NotificationRateLimit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_rate_limit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_rate_limit = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_AlertStrategy {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_AlertStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_AlertStrategy {}

impl BuildMonitoringAlertPolicy_AlertStrategy {
    pub fn build(self) -> MonitoringAlertPolicy_AlertStrategy {
        MonitoringAlertPolicy_AlertStrategy {
            auto_close: core::default::Default::default(),
            notification_prompts: core::default::Default::default(),
            notification_channel_strategy: core::default::Default::default(),
            notification_rate_limit: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_AlertStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_AlertStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_AlertStrategyRef {
        MonitoringAlertPolicy_AlertStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_AlertStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_close` after provisioning.\nIf an alert policy that was active has no data for this long, any open incidents will close."]
    pub fn auto_close(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_close", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notification_prompts` after provisioning.\nControl when notifications will be sent out. Possible values: [\"NOTIFICATION_PROMPT_UNSPECIFIED\", \"OPENED\", \"CLOSED\"]"]
    pub fn notification_prompts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_prompts", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_channel_strategy` after provisioning.\n"]
    pub fn notification_channel_strategy(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_AlertStrategy_NotificationChannelStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_channel_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_rate_limit` after provisioning.\n"]
    pub fn notification_rate_limit(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_AlertStrategy_NotificationRateLimitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_rate_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alignment_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_series_reducer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_by_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_series_aligner: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
    #[doc =
        "Set the field `alignment_period`.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn set_alignment_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alignment_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_series_reducer`.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn set_cross_series_reducer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_series_reducer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group_by_fields`.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn set_group_by_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group_by_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `per_series_aligner`.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn set_per_series_aligner(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.per_series_aligner = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {}

impl BuildMonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
        MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations {
            alignment_period: core::default::Default::default(),
            cross_series_reducer: core::default::Default::default(),
            group_by_fields: core::default::Default::default(),
            per_series_aligner: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef {
        MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alignment_period` after provisioning.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn alignment_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alignment_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_series_reducer` after provisioning.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn cross_series_reducer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_series_reducer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_by_fields` after provisioning.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn group_by_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_by_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `per_series_aligner` after provisioning.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn per_series_aligner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_series_aligner", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
    #[doc =
        "Set the field `count`.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent`.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {}

impl BuildMonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
        MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger {
            count: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef {
        MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nThe absolute number of time series\nthat must fail the predicate for the\ncondition to be triggered."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nThe percentage of time series that\nmust fail the predicate for the\ncondition to be triggered."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_Conditions_ConditionAbsentDynamic {
    aggregations: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    trigger: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsent {
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregations: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    dynamic: MonitoringAlertPolicy_Conditions_ConditionAbsentDynamic,
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsent {
    #[doc =
        "Set the field `filter`.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregations`.\n"]
    pub fn set_aggregations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionAbsent {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionAbsent {
    #[doc =
        "The amount of time that a time series must\nfail to report new data to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g. 60s, 120s, or 300s\n--are supported."]
    pub duration: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionAbsent {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionAbsent {
        MonitoringAlertPolicy_Conditions_ConditionAbsent {
            duration: self.duration,
            filter: core::default::Default::default(),
            aggregations: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionAbsentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionAbsentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionAbsentRef {
        MonitoringAlertPolicy_Conditions_ConditionAbsentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionAbsentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nfail to report new data to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g. 60s, 120s, or 300s\n--are supported."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregations` after provisioning.\n"]
    pub fn aggregations(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregations", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionMatchedLog {
    filter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_extractors: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionMatchedLog {
    #[doc =
        "Set the field `label_extractors`.\nA map from a label key to an extractor expression, which is used to\nextract the value for this label key. Each entry in this map is\na specification for how data should be extracted from log entries that\nmatch filter. Each combination of extracted values is treated as\na separate rule for the purposes of triggering notifications.\nLabel keys and corresponding values can be used in notifications\ngenerated by this condition."]
    pub fn set_label_extractors(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.label_extractors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionMatchedLog {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionMatchedLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionMatchedLog {
    #[doc = "A logs-based filter."]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionMatchedLog {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionMatchedLog {
        MonitoringAlertPolicy_Conditions_ConditionMatchedLog {
            filter: self.filter,
            label_extractors: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef {
        MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA logs-based filter."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key to an extractor expression, which is used to\nextract the value for this label key. Each entry in this map is\na specification for how data should be extracted from log entries that\nmatch filter. Each combination of extracted values is treated as\na separate rule for the purposes of triggering notifications.\nLabel keys and corresponding values can be used in notifications\ngenerated by this condition."]
    pub fn label_extractors(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.label_extractors", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageDynamic {
    trigger: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_missing_data: std::option::Option<terrars::PrimField<std::string::String>>,
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    dynamic: MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageDynamic,
}

impl MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
    #[doc =
        "Set the field `evaluation_missing_data`.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn set_evaluation_missing_data(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.evaluation_missing_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
    #[doc =
        "The amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub duration: terrars::PrimField<std::string::String>,
    #[doc = "Monitoring Query Language query that outputs a boolean stream."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
        MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage {
            duration: self.duration,
            evaluation_missing_data: core::default::Default::default(),
            query: self.query,
            trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef {
        MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_missing_data` after provisioning.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn evaluation_missing_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_missing_data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query` after provisioning.\nMonitoring Query Language query that outputs a boolean stream."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_metric_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
    #[doc =
        "Set the field `alert_rule`.\nThe alerting rule name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future.\n\nThis field is optional. If this field is not empty, then it must be a\nvalid Prometheus label name."]
    pub fn set_alert_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alert_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_metric_validation`.\nWhether to disable metric existence validation for this condition.\n\nThis allows alerting policies to be defined on metrics that do not yet\nexist, improving advanced customer workflows such as configuring\nalerting policies using Terraform.\n\nUsers with the 'monitoring.alertPolicyViewer' role are able to see the\nname of the non-existent metric in the alerting policy condition."]
    pub fn set_disable_metric_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_metric_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `duration`.\nAlerts are considered firing once their PromQL expression evaluated\nto be \"true\" for this long. Alerts whose PromQL expression was not\nevaluated to be \"true\" for long enough are considered pending. The\ndefault value is zero. Must be zero or positive."]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `evaluation_interval`.\nHow often this rule should be evaluated. Must be a positive multiple\nof 30 seconds or missing. The default value is 30 seconds. If this\nPrometheusQueryLanguageCondition was generated from a Prometheus\nalerting rule, then this value should be taken from the enclosing\nrule group."]
    pub fn set_evaluation_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.evaluation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels to add to or overwrite in the PromQL query result. Label names\nmust be valid.\n\nLabel values can be templatized by using variables. The only available\nvariable names are the names of the labels in the PromQL result,\nalthough label names beginning with \\_\\_ (two \"\\_\") are reserved for\ninternal use. \"labels\" may be empty. This field is intended to be used\nfor organizing and identifying the AlertPolicy."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rule_group`.\nThe rule group name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future. This field is optional."]
    pub fn set_rule_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
    #[doc =
        "The PromQL expression to evaluate. Every evaluation cycle this\nexpression is evaluated at the current time, and all resultant time\nseries become pending/firing alerts. This field must not be empty."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
        MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage {
            alert_rule: core::default::Default::default(),
            disable_metric_validation: core::default::Default::default(),
            duration: core::default::Default::default(),
            evaluation_interval: core::default::Default::default(),
            labels: core::default::Default::default(),
            query: self.query,
            rule_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef {
        MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alert_rule` after provisioning.\nThe alerting rule name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future.\n\nThis field is optional. If this field is not empty, then it must be a\nvalid Prometheus label name."]
    pub fn alert_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alert_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_metric_validation` after provisioning.\nWhether to disable metric existence validation for this condition.\n\nThis allows alerting policies to be defined on metrics that do not yet\nexist, improving advanced customer workflows such as configuring\nalerting policies using Terraform.\n\nUsers with the 'monitoring.alertPolicyViewer' role are able to see the\nname of the non-existent metric in the alerting policy condition."]
    pub fn disable_metric_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_metric_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nAlerts are considered firing once their PromQL expression evaluated\nto be \"true\" for this long. Alerts whose PromQL expression was not\nevaluated to be \"true\" for long enough are considered pending. The\ndefault value is zero. Must be zero or positive."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_interval` after provisioning.\nHow often this rule should be evaluated. Must be a positive multiple\nof 30 seconds or missing. The default value is 30 seconds. If this\nPrometheusQueryLanguageCondition was generated from a Prometheus\nalerting rule, then this value should be taken from the enclosing\nrule group."]
    pub fn evaluation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to add to or overwrite in the PromQL query result. Label names\nmust be valid.\n\nLabel values can be templatized by using variables. The only available\nvariable names are the names of the labels in the PromQL result,\nalthough label names beginning with \\_\\_ (two \"\\_\") are reserved for\ninternal use. \"labels\" may be empty. This field is intended to be used\nfor organizing and identifying the AlertPolicy."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query` after provisioning.\nThe PromQL expression to evaluate. Every evaluation cycle this\nexpression is evaluated at the current time, and all resultant time\nseries become pending/firing alerts. This field must not be empty."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_group` after provisioning.\nThe rule group name of this alert in the corresponding Prometheus\nconfiguration file.\n\nSome external tools may require this field to be populated correctly\nin order to refer to the original Prometheus configuration file.\nThe rule group name and the alert name are necessary to update the\nrelevant AlertPolicies in case the definition of the rule group changes\nin the future. This field is optional."]
    pub fn rule_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_group", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest {
    column: terrars::PrimField<std::string::String>,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest { }

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest {
    #[doc = "The name of the column containing the boolean value. If the value in a row is\nNULL, that row is ignored."]
    pub column: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest {
        MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest { column: self.column }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `column` after provisioning.\nThe name of the column containing the boolean value. If the value in a row is\nNULL, that row is ignored."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
    #[doc =
        "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal\nto 0 and typically must be less than or equal to 23. An API may\nchoose to allow the value \"24:00:00\" for scenarios like business\nclosing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and\nless than or equal to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or\nequal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and\ntypically must be less than or equal to 59. An API may allow the\nvalue 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
        MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal\nto 0 and typically must be less than or equal to 23. An API may\nchoose to allow the value \"24:00:00\" for scenarios like business\nclosing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and\nless than or equal to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or\nequal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and\ntypically must be less than or equal to 59. An API may allow the\nvalue 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_Conditions_ConditionSql_DailyDynamic {
    execution_time: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_Daily {
    periodicity: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_time: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime>>,
    dynamic: MonitoringAlertPolicy_Conditions_ConditionSql_DailyDynamic,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_Daily {
    #[doc = "Set the field `execution_time`.\n"]
    pub fn set_execution_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_Daily {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Daily>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_Daily {
    #[doc =
        "The number of days between runs. Must be greater than or equal\nto 1 day and less than or equal to 30 days."]
    pub periodicity: terrars::PrimField<f64>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_Daily {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_Daily {
        MonitoringAlertPolicy_Conditions_ConditionSql_Daily {
            periodicity: self.periodicity,
            execution_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `periodicity` after provisioning.\nThe number of days between runs. Must be greater than or equal\nto 1 day and less than or equal to 30 days."]
    pub fn periodicity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_time` after provisioning.\n"]
    pub fn execution_time(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_Daily_ExecutionTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_offset: std::option::Option<terrars::PrimField<f64>>,
    periodicity: terrars::PrimField<f64>,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
    #[doc =
        "Set the field `minute_offset`.\nThe number of minutes after the hour (in UTC) to run the query.\nMust be greater than or equal to 0 minutes and less than or equal to\n59 minutes.  If left unspecified, then an arbitrary offset is used."]
    pub fn set_minute_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Hourly>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
    #[doc =
        "Number of hours between runs. The interval must be greater than or\nequal to 1 hour and less than or equal to 48 hours."]
    pub periodicity: terrars::PrimField<f64>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
        MonitoringAlertPolicy_Conditions_ConditionSql_Hourly {
            minute_offset: core::default::Default::default(),
            periodicity: self.periodicity,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `minute_offset` after provisioning.\nThe number of minutes after the hour (in UTC) to run the query.\nMust be greater than or equal to 0 minutes and less than or equal to\n59 minutes.  If left unspecified, then an arbitrary offset is used."]
    pub fn minute_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `periodicity` after provisioning.\nNumber of hours between runs. The interval must be greater than or\nequal to 1 hour and less than or equal to 48 hours."]
    pub fn periodicity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_Minutes {
    periodicity: terrars::PrimField<f64>,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_Minutes { }

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_Minutes {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Minutes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_Minutes {
    #[doc =
        "Number of minutes between runs. The interval must be greater than or\nequal to 5 minutes and less than or equal to 1440 minutes."]
    pub periodicity: terrars::PrimField<f64>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_Minutes {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_Minutes {
        MonitoringAlertPolicy_Conditions_ConditionSql_Minutes { periodicity: self.periodicity }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `periodicity` after provisioning.\nNumber of minutes between runs. The interval must be greater than or\nequal to 5 minutes and less than or equal to 1440 minutes."]
    pub fn periodicity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.periodicity", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
    comparison: terrars::PrimField<std::string::String>,
    threshold: terrars::PrimField<f64>,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest { }

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
    #[doc =
        "The comparison to apply between the time series\n(indicated by filter and aggregation) and the\nthreshold (indicated by threshold_value). The\ncomparison is applied on each time series, with\nthe time series on the left-hand side and the\nthreshold on the right-hand side.\n\nThe Cloud Monitoring API only supports\n'COMPARISON_LT' and 'COMPARISON_GT' for SQL\nrow-count thresholds; the other values are kept\nin the schema for backward compatibility with\nimported state but will be rejected by the API.\nSee\nhttps://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.alertPolicies#MetricThreshold. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = "The value against which to compare the row count."]
    pub threshold: terrars::PrimField<f64>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
        MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest {
            comparison: self.comparison,
            threshold: self.threshold,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef {
        MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison` after provisioning.\nThe comparison to apply between the time series\n(indicated by filter and aggregation) and the\nthreshold (indicated by threshold_value). The\ncomparison is applied on each time series, with\nthe time series on the left-hand side and the\nthreshold on the right-hand side.\n\nThe Cloud Monitoring API only supports\n'COMPARISON_LT' and 'COMPARISON_GT' for SQL\nrow-count thresholds; the other values are kept\nin the schema for backward compatibility with\nimported state but will be rejected by the API.\nSee\nhttps://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.alertPolicies#MetricThreshold. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe value against which to compare the row count."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_Conditions_ConditionSqlDynamic {
    boolean_test: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest>>,
    daily: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_Daily>>,
    hourly: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_Hourly>>,
    minutes: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_Minutes>>,
    row_count_test: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSql {
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_test: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_Daily>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_Hourly>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_Minutes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_count_test: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest>>,
    dynamic: MonitoringAlertPolicy_Conditions_ConditionSqlDynamic,
}

impl MonitoringAlertPolicy_Conditions_ConditionSql {
    #[doc = "Set the field `boolean_test`.\n"]
    pub fn set_boolean_test(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boolean_test = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boolean_test = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `daily`.\n"]
    pub fn set_daily(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Daily>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.daily = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.daily = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hourly`.\n"]
    pub fn set_hourly(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Hourly>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hourly = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hourly = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_Minutes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.minutes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.minutes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_count_test`.\n"]
    pub fn set_row_count_test(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_count_test = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_count_test = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionSql {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionSql {
    #[doc =
        "The Log Analytics SQL query to run, as a string.  The query must\nconform to the required shape. Specifically, the query must not try to\nfilter the input by time.  A filter will automatically be applied\nto filter the input so that the query receives all rows received\nsince the last time the query was run."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionSql {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionSql {
        MonitoringAlertPolicy_Conditions_ConditionSql {
            query: self.query,
            boolean_test: core::default::Default::default(),
            daily: core::default::Default::default(),
            hourly: core::default::Default::default(),
            minutes: core::default::Default::default(),
            row_count_test: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionSqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionSqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionSqlRef {
        MonitoringAlertPolicy_Conditions_ConditionSqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionSqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query` after provisioning.\nThe Log Analytics SQL query to run, as a string.  The query must\nconform to the required shape. Specifically, the query must not try to\nfilter the input by time.  A filter will automatically be applied\nto filter the input so that the query receives all rows received\nsince the last time the query was run."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `boolean_test` after provisioning.\n"]
    pub fn boolean_test(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_BooleanTestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_test", self.base))
    }

    #[doc = "Get a reference to the value of field `daily` after provisioning.\n"]
    pub fn daily(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_DailyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.daily", self.base))
    }

    #[doc = "Get a reference to the value of field `hourly` after provisioning.\n"]
    pub fn hourly(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_HourlyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hourly", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_MinutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `row_count_test` after provisioning.\n"]
    pub fn row_count_test(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSql_RowCountTestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_count_test", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alignment_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_series_reducer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_by_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_series_aligner: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
    #[doc =
        "Set the field `alignment_period`.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn set_alignment_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alignment_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_series_reducer`.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn set_cross_series_reducer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_series_reducer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group_by_fields`.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn set_group_by_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.group_by_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `per_series_aligner`.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn set_per_series_aligner(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.per_series_aligner = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {}

impl BuildMonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
        MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations {
            alignment_period: core::default::Default::default(),
            cross_series_reducer: core::default::Default::default(),
            group_by_fields: core::default::Default::default(),
            per_series_aligner: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef {
        MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alignment_period` after provisioning.\nThe alignment period for per-time\nseries alignment. If present,\nalignmentPeriod must be at least\n60 seconds. After per-time series\nalignment, each time series will\ncontain data points only on the\nperiod boundaries. If\nperSeriesAligner is not specified\nor equals ALIGN_NONE, then this\nfield is ignored. If\nperSeriesAligner is specified and\ndoes not equal ALIGN_NONE, then\nthis field must be defined;\notherwise an error is returned."]
    pub fn alignment_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alignment_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_series_reducer` after provisioning.\nThe approach to be used to combine\ntime series. Not all reducer\nfunctions may be applied to all\ntime series, depending on the\nmetric type and the value type of\nthe original time series.\nReduction may change the metric\ntype of value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"REDUCE_NONE\", \"REDUCE_MEAN\", \"REDUCE_MIN\", \"REDUCE_MAX\", \"REDUCE_SUM\", \"REDUCE_STDDEV\", \"REDUCE_COUNT\", \"REDUCE_COUNT_TRUE\", \"REDUCE_COUNT_FALSE\", \"REDUCE_FRACTION_TRUE\", \"REDUCE_PERCENTILE_99\", \"REDUCE_PERCENTILE_95\", \"REDUCE_PERCENTILE_50\", \"REDUCE_PERCENTILE_05\"]"]
    pub fn cross_series_reducer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_series_reducer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_by_fields` after provisioning.\nThe set of fields to preserve when\ncrossSeriesReducer is specified.\nThe groupByFields determine how\nthe time series are partitioned\ninto subsets prior to applying the\naggregation function. Each subset\ncontains time series that have the\nsame value for each of the\ngrouping fields. Each individual\ntime series is a member of exactly\none subset. The crossSeriesReducer\nis applied to each subset of time\nseries. It is not possible to\nreduce across different resource\ntypes, so this field implicitly\ncontains resource.type. Fields not\nspecified in groupByFields are\naggregated away. If groupByFields\nis not specified and all the time\nseries have the same resource\ntype, then the time series are\naggregated into a single output\ntime series. If crossSeriesReducer\nis not defined, this field is\nignored."]
    pub fn group_by_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_by_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `per_series_aligner` after provisioning.\nThe approach to be used to align\nindividual time series. Not all\nalignment functions may be applied\nto all time series, depending on\nthe metric type and value type of\nthe original time series.\nAlignment may change the metric\ntype or the value type of the time\nseries.Time series data must be\naligned in order to perform cross-\ntime series reduction. If\ncrossSeriesReducer is specified,\nthen perSeriesAligner must be\nspecified and not equal ALIGN_NONE\nand alignmentPeriod must be\nspecified; otherwise, an error is\nreturned. Possible values: [\"ALIGN_NONE\", \"ALIGN_DELTA\", \"ALIGN_RATE\", \"ALIGN_INTERPOLATE\", \"ALIGN_NEXT_OLDER\", \"ALIGN_MIN\", \"ALIGN_MAX\", \"ALIGN_MEAN\", \"ALIGN_COUNT\", \"ALIGN_SUM\", \"ALIGN_STDDEV\", \"ALIGN_COUNT_TRUE\", \"ALIGN_COUNT_FALSE\", \"ALIGN_FRACTION_TRUE\", \"ALIGN_PERCENTILE_99\", \"ALIGN_PERCENTILE_95\", \"ALIGN_PERCENTILE_50\", \"ALIGN_PERCENTILE_05\", \"ALIGN_PERCENT_CHANGE\"]"]
    pub fn per_series_aligner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_series_aligner", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
    forecast_horizon: terrars::PrimField<std::string::String>,
}

impl MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions { }

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
    #[doc =
        "The length of time into the future to forecast\nwhether a timeseries will violate the threshold.\nIf the predicted value is found to violate the\nthreshold, and the violation is observed in all\nforecasts made for the Configured 'duration',\nthen the timeseries is considered to be failing."]
    pub forecast_horizon: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
        MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions {
            forecast_horizon: self.forecast_horizon,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef {
        MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `forecast_horizon` after provisioning.\nThe length of time into the future to forecast\nwhether a timeseries will violate the threshold.\nIf the predicted value is found to violate the\nthreshold, and the violation is observed in all\nforecasts made for the Configured 'duration',\nthen the timeseries is considered to be failing."]
    pub fn forecast_horizon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forecast_horizon", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_Conditions_ConditionThresholdDynamic {
    aggregations: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    denominator_aggregations: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations>>,
    forecast_options: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions>>,
    trigger: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThreshold {
    comparison: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denominator_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_missing_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregations: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denominator_aggregations: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forecast_options: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    dynamic: MonitoringAlertPolicy_Conditions_ConditionThresholdDynamic,
}

impl MonitoringAlertPolicy_Conditions_ConditionThreshold {
    #[doc =
        "Set the field `denominator_filter`.\nA filter that identifies a time series that\nshould be used as the denominator of a ratio\nthat will be compared with the threshold. If\na denominator_filter is specified, the time\nseries specified by the filter field will be\nused as the numerator.The filter is similar\nto the one that is specified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_denominator_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.denominator_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `evaluation_missing_data`.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn set_evaluation_missing_data(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.evaluation_missing_data = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_value`.\nA value against which to compare the time\nseries."]
    pub fn set_threshold_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregations`.\n"]
    pub fn set_aggregations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Aggregations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `denominator_aggregations`.\n"]
    pub fn set_denominator_aggregations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.denominator_aggregations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.denominator_aggregations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forecast_options`.\n"]
    pub fn set_forecast_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forecast_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forecast_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent_Trigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions_ConditionThreshold {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions_ConditionThreshold {
    #[doc =
        "The comparison to apply between the time series\n(indicated by filter and aggregation) and the threshold\n(indicated by threshold_value). The comparison is\napplied on each time series, with the time series on\nthe left-hand side and the threshold on the right-hand\nside.\n\nThe Cloud Monitoring API only supports 'COMPARISON_LT'\nand 'COMPARISON_GT' for metric-threshold conditions; the\nother values are kept in the schema for backward\ncompatibility with imported state but will be rejected\nby the API. See\nhttps://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.alertPolicies#MetricThreshold. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc =
        "The amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub duration: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions_ConditionThreshold {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions_ConditionThreshold {
        MonitoringAlertPolicy_Conditions_ConditionThreshold {
            comparison: self.comparison,
            denominator_filter: core::default::Default::default(),
            duration: self.duration,
            evaluation_missing_data: core::default::Default::default(),
            filter: core::default::Default::default(),
            threshold_value: core::default::Default::default(),
            aggregations: core::default::Default::default(),
            denominator_aggregations: core::default::Default::default(),
            forecast_options: core::default::Default::default(),
            trigger: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Conditions_ConditionThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Conditions_ConditionThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringAlertPolicy_Conditions_ConditionThresholdRef {
        MonitoringAlertPolicy_Conditions_ConditionThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Conditions_ConditionThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `comparison` after provisioning.\nThe comparison to apply between the time series\n(indicated by filter and aggregation) and the threshold\n(indicated by threshold_value). The comparison is\napplied on each time series, with the time series on\nthe left-hand side and the threshold on the right-hand\nside.\n\nThe Cloud Monitoring API only supports 'COMPARISON_LT'\nand 'COMPARISON_GT' for metric-threshold conditions; the\nother values are kept in the schema for backward\ncompatibility with imported state but will be rejected\nby the API. See\nhttps://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.alertPolicies#MetricThreshold. Possible values: [\"COMPARISON_GT\", \"COMPARISON_GE\", \"COMPARISON_LT\", \"COMPARISON_LE\", \"COMPARISON_EQ\", \"COMPARISON_NE\"]"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc =
        "Get a reference to the value of field `denominator_filter` after provisioning.\nA filter that identifies a time series that\nshould be used as the denominator of a ratio\nthat will be compared with the threshold. If\na denominator_filter is specified, the time\nseries specified by the filter field will be\nused as the numerator.The filter is similar\nto the one that is specified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn denominator_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.denominator_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nThe amount of time that a time series must\nviolate the threshold to be considered\nfailing. Currently, only values that are a\nmultiple of a minute--e.g., 0, 60, 120, or\n300 seconds--are supported. If an invalid\nvalue is given, an error will be returned.\nWhen choosing a duration, it is useful to\nkeep in mind the frequency of the underlying\ntime series data (which may also be affected\nby any alignments specified in the\naggregations field); a good duration is long\nenough so that a single outlier does not\ngenerate spurious alerts, but short enough\nthat unhealthy states are detected and\nalerted on quickly."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_missing_data` after provisioning.\nA condition control that determines how\nmetric-threshold conditions are evaluated when\ndata stops arriving. Possible values: [\"EVALUATION_MISSING_DATA_INACTIVE\", \"EVALUATION_MISSING_DATA_ACTIVE\", \"EVALUATION_MISSING_DATA_NO_OP\"]"]
    pub fn evaluation_missing_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_missing_data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nA filter that identifies which time series\nshould be compared with the threshold.The\nfilter is similar to the one that is\nspecified in the\nMetricService.ListTimeSeries request (that\ncall is useful to verify the time series\nthat will be retrieved / processed) and must\nspecify the metric type and optionally may\ncontain restrictions on resource type,\nresource labels, and metric labels. This\nfield may not exceed 2048 Unicode characters\nin length."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threshold_value` after provisioning.\nA value against which to compare the time\nseries."]
    pub fn threshold_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_value", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregations` after provisioning.\n"]
    pub fn aggregations(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsent_AggregationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregations", self.base))
    }

    #[doc = "Get a reference to the value of field `denominator_aggregations` after provisioning.\n"]
    pub fn denominator_aggregations(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionThreshold_DenominatorAggregationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denominator_aggregations", self.base))
    }

    #[doc = "Get a reference to the value of field `forecast_options` after provisioning.\n"]
    pub fn forecast_options(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionThreshold_ForecastOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forecast_options", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsent_TriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_ConditionsDynamic {
    condition_absent: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionAbsent>>,
    condition_matched_log: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionMatchedLog>>,
    condition_monitoring_query_language: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage>>,
    condition_prometheus_query_language: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage>>,
    condition_sql: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionSql>>,
    condition_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Conditions_ConditionThreshold>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Conditions {
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_absent: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionAbsent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_matched_log: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionMatchedLog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_monitoring_query_language: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_prometheus_query_language: std
    ::option
    ::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_sql: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionSql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_threshold: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Conditions_ConditionThreshold>>,
    dynamic: MonitoringAlertPolicy_ConditionsDynamic,
}

impl MonitoringAlertPolicy_Conditions {
    #[doc = "Set the field `condition_absent`.\n"]
    pub fn set_condition_absent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionAbsent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_absent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_absent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_matched_log`.\n"]
    pub fn set_condition_matched_log(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionMatchedLog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_matched_log = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_matched_log = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_monitoring_query_language`.\n"]
    pub fn set_condition_monitoring_query_language(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_monitoring_query_language = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_monitoring_query_language = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_prometheus_query_language`.\n"]
    pub fn set_condition_prometheus_query_language(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_prometheus_query_language = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_prometheus_query_language = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_sql`.\n"]
    pub fn set_condition_sql(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionSql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_sql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_sql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_threshold`.\n"]
    pub fn set_condition_threshold(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Conditions_ConditionThreshold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_threshold = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Conditions {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Conditions {
    #[doc =
        "A short name or phrase used to identify the\ncondition in dashboards, notifications, and\nincidents. To avoid confusion, don't use the same\ndisplay name for multiple conditions in the same\npolicy."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringAlertPolicy_Conditions {
    pub fn build(self) -> MonitoringAlertPolicy_Conditions {
        MonitoringAlertPolicy_Conditions {
            display_name: self.display_name,
            condition_absent: core::default::Default::default(),
            condition_matched_log: core::default::Default::default(),
            condition_monitoring_query_language: core::default::Default::default(),
            condition_prometheus_query_language: core::default::Default::default(),
            condition_sql: core::default::Default::default(),
            condition_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_ConditionsRef {
        MonitoringAlertPolicy_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA short name or phrase used to identify the\ncondition in dashboards, notifications, and\nincidents. To avoid confusion, don't use the same\ndisplay name for multiple conditions in the same\npolicy."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique resource name for this condition.\nIts syntax is:\nprojects/[PROJECT_ID]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]\n[CONDITION_ID] is assigned by Stackdriver Monitoring when\nthe condition is created as part of a new or updated alerting\npolicy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_absent` after provisioning.\n"]
    pub fn condition_absent(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionAbsentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_absent", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_matched_log` after provisioning.\n"]
    pub fn condition_matched_log(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionMatchedLogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_matched_log", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_monitoring_query_language` after provisioning.\n"]
    pub fn condition_monitoring_query_language(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionMonitoringQueryLanguageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_monitoring_query_language", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_prometheus_query_language` after provisioning.\n"]
    pub fn condition_prometheus_query_language(
        &self,
    ) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionPrometheusQueryLanguageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_prometheus_query_language", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_sql` after provisioning.\n"]
    pub fn condition_sql(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionSqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_sql", self.base))
    }

    #[doc = "Get a reference to the value of field `condition_threshold` after provisioning.\n"]
    pub fn condition_threshold(&self) -> terrars::ListRef<MonitoringAlertPolicy_Conditions_ConditionThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Documentation_Links {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_Documentation_Links {
    #[doc =
        "Set the field `display_name`.\nA short display name for the link. The display name must not be empty or exceed 63 characters. Example: \"playbook\"."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url`.\nThe url of a webpage. A url can be templatized by using variables in the path or the query parameters. The total length of a URL should not exceed 2083 characters before and after variable expansion. Example: \"https://my_domain.com/playbook?name=${resource.name}\"."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Documentation_Links {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Documentation_Links>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Documentation_Links {}

impl BuildMonitoringAlertPolicy_Documentation_Links {
    pub fn build(self) -> MonitoringAlertPolicy_Documentation_Links {
        MonitoringAlertPolicy_Documentation_Links {
            display_name: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_Documentation_LinksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_Documentation_LinksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_Documentation_LinksRef {
        MonitoringAlertPolicy_Documentation_LinksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_Documentation_LinksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA short display name for the link. The display name must not be empty or exceed 63 characters. Example: \"playbook\"."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe url of a webpage. A url can be templatized by using variables in the path or the query parameters. The total length of a URL should not exceed 2083 characters before and after variable expansion. Example: \"https://my_domain.com/playbook?name=${resource.name}\"."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringAlertPolicy_DocumentationDynamic {
    links: std::option::Option<terrars::DynamicBlock<MonitoringAlertPolicy_Documentation_Links>>,
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Documentation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mime_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    links: std::option::Option<std::vec::Vec<MonitoringAlertPolicy_Documentation_Links>>,
    dynamic: MonitoringAlertPolicy_DocumentationDynamic,
}

impl MonitoringAlertPolicy_Documentation {
    #[doc =
        "Set the field `content`.\nThe text of the documentation, interpreted according to mimeType.\nThe content may not exceed 8,192 Unicode characters and may not\nexceed more than 10,240 bytes when encoded in UTF-8 format,\nwhichever is smaller."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mime_type`.\nThe format of the content field. Presently, only the value\n\"text/markdown\" is supported."]
    pub fn set_mime_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mime_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subject`.\nThe subject line of the notification. The subject line may not\nexceed 10,240 bytes. In notifications generated by this policy the contents\nof the subject line after variable expansion will be truncated to 255 bytes\nor shorter at the latest UTF-8 character boundary."]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `links`.\n"]
    pub fn set_links(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringAlertPolicy_Documentation_Links>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.links = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.links = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringAlertPolicy_Documentation {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Documentation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Documentation {}

impl BuildMonitoringAlertPolicy_Documentation {
    pub fn build(self) -> MonitoringAlertPolicy_Documentation {
        MonitoringAlertPolicy_Documentation {
            content: core::default::Default::default(),
            mime_type: core::default::Default::default(),
            subject: core::default::Default::default(),
            links: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_DocumentationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_DocumentationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_DocumentationRef {
        MonitoringAlertPolicy_DocumentationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_DocumentationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe text of the documentation, interpreted according to mimeType.\nThe content may not exceed 8,192 Unicode characters and may not\nexceed more than 10,240 bytes when encoded in UTF-8 format,\nwhichever is smaller."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nThe format of the content field. Presently, only the value\n\"text/markdown\" is supported."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject` after provisioning.\nThe subject line of the notification. The subject line may not\nexceed 10,240 bytes. In notifications generated by this policy the contents\nof the subject line after variable expansion will be truncated to 255 bytes\nor shorter at the latest UTF-8 character boundary."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(&self) -> terrars::ListRef<MonitoringAlertPolicy_Documentation_LinksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringAlertPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringAlertPolicy_Timeouts {
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

impl terrars::ToListMappable for MonitoringAlertPolicy_Timeouts {
    type O = terrars::BlockAssignable<MonitoringAlertPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringAlertPolicy_Timeouts {}

impl BuildMonitoringAlertPolicy_Timeouts {
    pub fn build(self) -> MonitoringAlertPolicy_Timeouts {
        MonitoringAlertPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringAlertPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringAlertPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringAlertPolicy_TimeoutsRef {
        MonitoringAlertPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringAlertPolicy_TimeoutsRef {
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
