#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ParallelstoreInstanceData_ {
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
    capacity_gib: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_stripe_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_stripe_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ParallelstoreInstance_Timeouts>,
}

struct ParallelstoreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ParallelstoreInstanceData_>,
}

#[derive(Clone)]
pub struct ParallelstoreInstance(std::rc::Rc<ParallelstoreInstance_>);

impl ParallelstoreInstance {
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

    #[doc =
        "Set the field `deployment_type`.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn set_deployment_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the instance. 2048 characters or less."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `directory_stripe_level`.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn set_directory_stripe_level(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().directory_stripe_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_stripe_level`.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn set_file_stripe_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().file_stripe_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_ip_range`.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn set_reserved_ip_range(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reserved_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ParallelstoreInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_points` after provisioning.\nOutput only. List of access_points.\nContains a list of IPv4 addresses used for client side configuration."]
    pub fn access_points(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_points", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nRequired. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `daos_version` after provisioning.\nThe version of DAOS software running in the instance."]
    pub fn daos_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daos_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_type` after provisioning.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the instance. 2048 characters or less."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `directory_stripe_level` after provisioning.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn directory_stripe_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_stripe_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address\nrange associated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. This field is populated by the service\nand contains the value currently used by the service."]
    pub fn effective_reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_stripe_level` after provisioning.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn file_stripe_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_stripe_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the instance, in the format\n'projects/{project}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe instance state.\n  Possible values:\n  STATE_UNSPECIFIED\n  CREATING\n  ACTIVE\n  DELETING\n  FAILED\n  UPGRADING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ParallelstoreInstance_TimeoutsRef {
        ParallelstoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ParallelstoreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ParallelstoreInstance { }

impl terrars::ToListMappable for ParallelstoreInstance {
    type O = terrars::ListRef<ParallelstoreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ParallelstoreInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_parallelstore_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildParallelstoreInstance {
    pub tf_id: std::string::String,
    #[doc = "Required. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub capacity_gib: terrars::PrimField<std::string::String>,
    #[doc =
        "The logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildParallelstoreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> ParallelstoreInstance {
        let out = ParallelstoreInstance(std::rc::Rc::new(ParallelstoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ParallelstoreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                description: core::default::Default::default(),
                directory_stripe_level: core::default::Default::default(),
                file_stripe_level: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                reserved_ip_range: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ParallelstoreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ParallelstoreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ParallelstoreInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_points` after provisioning.\nOutput only. List of access_points.\nContains a list of IPv4 addresses used for client side configuration."]
    pub fn access_points(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_points", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nRequired. Immutable. Storage capacity of Parallelstore instance in Gibibytes (GiB)."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `daos_version` after provisioning.\nThe version of DAOS software running in the instance."]
    pub fn daos_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daos_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_type` after provisioning.\nParallelstore Instance deployment type.\n  Possible values:\n  DEPLOYMENT_TYPE_UNSPECIFIED\n  SCRATCH\n  PERSISTENT"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the instance. 2048 characters or less."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `directory_stripe_level` after provisioning.\nStripe level for directories.\nMIN when directory has a small number of files.\nMAX when directory has a large number of files.\n  Possible values:\n  DIRECTORY_STRIPE_LEVEL_UNSPECIFIED\n  DIRECTORY_STRIPE_LEVEL_MIN\n  DIRECTORY_STRIPE_LEVEL_BALANCED\n  DIRECTORY_STRIPE_LEVEL_MAX"]
    pub fn directory_stripe_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_stripe_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address\nrange associated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. This field is populated by the service\nand contains the value currently used by the service."]
    pub fn effective_reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_stripe_level` after provisioning.\nStripe level for files.\nMIN better suited for small size files.\nMAX higher throughput performance for larger files.\n  Possible values:\n  FILE_STRIPE_LEVEL_UNSPECIFIED\n  FILE_STRIPE_LEVEL_MIN\n  FILE_STRIPE_LEVEL_BALANCED\n  FILE_STRIPE_LEVEL_MAX"]
    pub fn file_stripe_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_stripe_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe logical name of the Parallelstore instance in the user project with the following restrictions:\n  * Must contain only lowercase letters, numbers, and hyphens.\n  * Must start with a letter.\n  * Must be between 1-63 characters.\n  * Must end with a number or a letter.\n  * Must be unique within the customer project/ location"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nCloud Labels are a flexible and lightweight mechanism for\norganizing cloud resources into groups that reflect a customer's organizational\nneeds and deployment strategies. Cloud Labels can be used to filter collections\nof resources. They can be used to control how resource metrics are aggregated.\nAnd they can be used as arguments to policy management rules (e.g. route, firewall,\nload balancing, etc.).\n\n* Label keys must be between 1 and 63 characters long and must conform to\n the following regular expression: 'a-z{0,62}'.\n* Label values must be between 0 and 63 characters long and must conform\n to the regular expression '[a-z0-9_-]{0,63}'.\n* No more than 64 labels can be associated with a given resource.\n\nSee https://goo.gl/xmQnxf for more information on and examples of labels.\n\nIf you plan to use labels in your own code, please note that additional\ncharacters may be allowed in the future. Therefore, you are advised to use\nan internal label representation, such as JSON, which doesn't rely upon\nspecific characters being disallowed.  For example, representing labels\nas the string:  'name + \"_\" + value' would prove problematic if we were to\nallow '\"_\"' in a future release. \"\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the instance, in the format\n'projects/{project}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nImmutable. The name of the Google Compute Engine [VPC network](https://cloud.google.com/vpc/docs/vpc)\nto which the instance is connected."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_range` after provisioning.\nImmutable. Contains the id of the allocated IP address range\nassociated with the private service access connection for example, \\\"test-default\\\"\nassociated with IP range 10.0.0.0/29. If no range id is provided all ranges will\nbe considered."]
    pub fn reserved_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe instance state.\n  Possible values:\n  STATE_UNSPECIFIED\n  CREATING\n  ACTIVE\n  DELETING\n  FAILED\n  UPGRADING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the instance was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ParallelstoreInstance_TimeoutsRef {
        ParallelstoreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ParallelstoreInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ParallelstoreInstance_Timeouts {
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

impl terrars::ToListMappable for ParallelstoreInstance_Timeouts {
    type O = terrars::BlockAssignable<ParallelstoreInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildParallelstoreInstance_Timeouts {}

impl BuildParallelstoreInstance_Timeouts {
    pub fn build(self) -> ParallelstoreInstance_Timeouts {
        ParallelstoreInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ParallelstoreInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ParallelstoreInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ParallelstoreInstance_TimeoutsRef {
        ParallelstoreInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ParallelstoreInstance_TimeoutsRef {
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
