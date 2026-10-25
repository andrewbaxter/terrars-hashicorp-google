#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AlloydbInstanceData_ {
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
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_type: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_connection_config: std::option::Option<std::vec::Vec<AlloydbInstance_ClientConnectionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool_config: std::option::Option<std::vec::Vec<AlloydbInstance_ConnectionPoolConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_config: std::option::Option<std::vec::Vec<AlloydbInstance_MachineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<AlloydbInstance_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_instance_config: std::option::Option<std::vec::Vec<AlloydbInstance_PscInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_insights_config: std::option::Option<std::vec::Vec<AlloydbInstance_QueryInsightsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_pool_config: std::option::Option<std::vec::Vec<AlloydbInstance_ReadPoolConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AlloydbInstance_Timeouts>,
    dynamic: AlloydbInstanceDynamic,
}

struct AlloydbInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AlloydbInstanceData_>,
}

#[derive(Clone)]
pub struct AlloydbInstance(std::rc::Rc<AlloydbInstance_>);

impl AlloydbInstance {
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
        "Set the field `activation_policy`.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn set_activation_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().activation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `annotations`.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `availability_type`.\n'Availability type of a primary Instance. Defaults to REGIONAL.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances\nderive their availability from 'read_pool_config.node_count': a pool\nwith one node is ZONAL, while a pool with two or more nodes is REGIONAL.\nRead Pool instances do not support setting this field directly.\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn set_availability_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_flags`.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn set_database_flags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().database_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-settable and human-readable display name for the Instance."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gce_zone`.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn set_gce_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gce_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_connection_config`.\n"]
    pub fn set_client_connection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_ClientConnectionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_connection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_connection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_ConnectionPoolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_config`.\n"]
    pub fn set_machine_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_MachineConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().machine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.machine_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_instance_config`.\n"]
    pub fn set_psc_instance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_insights_config`.\n"]
    pub fn set_query_insights_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_QueryInsightsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_insights_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_insights_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `read_pool_config`.\n"]
    pub fn set_read_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_ReadPoolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().read_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.read_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AlloydbInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of a primary Instance. Defaults to REGIONAL.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances\nderive their availability from 'read_pool_config.node_count': a pool\nwith one node is ZONAL, while a pool with two or more nodes is REGIONAL.\nRead Pool instances do not support setting this field directly.\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
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
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_public_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> terrars::ListRef<AlloydbInstance_ClientConnectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> terrars::ListRef<AlloydbInstance_ConnectionPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> terrars::ListRef<AlloydbInstance_MachineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\n"]
    pub fn psc_instance_config(&self) -> terrars::ListRef<AlloydbInstance_PscInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> terrars::ListRef<AlloydbInstance_QueryInsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> terrars::ListRef<AlloydbInstance_ReadPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstance_TimeoutsRef {
        AlloydbInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AlloydbInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AlloydbInstance { }

impl terrars::ToListMappable for AlloydbInstance {
    type O = terrars::ListRef<AlloydbInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AlloydbInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_alloydb_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAlloydbInstance {
    pub tf_id: std::string::String,
    #[doc =
        "Identifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the alloydb instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildAlloydbInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> AlloydbInstance {
        let out = AlloydbInstance(std::rc::Rc::new(AlloydbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AlloydbInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                activation_policy: core::default::Default::default(),
                annotations: core::default::Default::default(),
                availability_type: core::default::Default::default(),
                cluster: self.cluster,
                database_flags: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                gce_zone: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                instance_type: self.instance_type,
                labels: core::default::Default::default(),
                client_connection_config: core::default::Default::default(),
                connection_pool_config: core::default::Default::default(),
                machine_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                psc_instance_config: core::default::Default::default(),
                query_insights_config: core::default::Default::default(),
                read_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AlloydbInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AlloydbInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of a primary Instance. Defaults to REGIONAL.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances\nderive their availability from 'read_pool_config.node_count': a pool\nwith one node is ZONAL, while a pool with two or more nodes is REGIONAL.\nRead Pool instances do not support setting this field directly.\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
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
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_public_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> terrars::ListRef<AlloydbInstance_ClientConnectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> terrars::ListRef<AlloydbInstance_ConnectionPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> terrars::ListRef<AlloydbInstance_MachineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<AlloydbInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\n"]
    pub fn psc_instance_config(&self) -> terrars::ListRef<AlloydbInstance_PscInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> terrars::ListRef<AlloydbInstance_QueryInsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> terrars::ListRef<AlloydbInstance_ReadPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstance_TimeoutsRef {
        AlloydbInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AlloydbInstanceDynamic {
    client_connection_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_ClientConnectionConfig>>,
    connection_pool_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_ConnectionPoolConfig>>,
    machine_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_MachineConfig>>,
    network_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_NetworkConfig>>,
    psc_instance_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_PscInstanceConfig>>,
    query_insights_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_QueryInsightsConfig>>,
    read_pool_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_ReadPoolConfig>>,
}

#[derive(Serialize)]
pub struct AlloydbInstance_ClientConnectionConfig_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_ClientConnectionConfig_SslConfig {
    #[doc =
        "Set the field `ssl_mode`.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_ClientConnectionConfig_SslConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_ClientConnectionConfig_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_ClientConnectionConfig_SslConfig {}

impl BuildAlloydbInstance_ClientConnectionConfig_SslConfig {
    pub fn build(self) -> AlloydbInstance_ClientConnectionConfig_SslConfig {
        AlloydbInstance_ClientConnectionConfig_SslConfig { ssl_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_ClientConnectionConfig_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_ClientConnectionConfig_SslConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbInstance_ClientConnectionConfig_SslConfigRef {
        AlloydbInstance_ClientConnectionConfig_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_ClientConnectionConfig_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbInstance_ClientConnectionConfigDynamic {
    ssl_config: std::option::Option<terrars::DynamicBlock<AlloydbInstance_ClientConnectionConfig_SslConfig>>,
}

#[derive(Serialize)]
pub struct AlloydbInstance_ClientConnectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_connectors: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<AlloydbInstance_ClientConnectionConfig_SslConfig>>,
    dynamic: AlloydbInstance_ClientConnectionConfigDynamic,
}

impl AlloydbInstance_ClientConnectionConfig {
    #[doc =
        "Set the field `require_connectors`.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn set_require_connectors(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_connectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_ClientConnectionConfig_SslConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_ClientConnectionConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_ClientConnectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_ClientConnectionConfig {}

impl BuildAlloydbInstance_ClientConnectionConfig {
    pub fn build(self) -> AlloydbInstance_ClientConnectionConfig {
        AlloydbInstance_ClientConnectionConfig {
            require_connectors: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_ClientConnectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_ClientConnectionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_ClientConnectionConfigRef {
        AlloydbInstance_ClientConnectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_ClientConnectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `require_connectors` after provisioning.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn require_connectors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_connectors", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<AlloydbInstance_ClientConnectionConfig_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_ConnectionPoolConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl AlloydbInstance_ConnectionPoolConfig {
    #[doc =
        "Set the field `flags`.\nFlags for configuring managed connection pooling when it is enabled.\nThese flags will only be set if 'connection_pool_config.enabled' is\ntrue.\nPlease see\nhttps://cloud.google.com/alloydb/docs/configure-managed-connection-pooling#configuration-options\nfor a comprehensive list of flags that can be set. To specify the flags\nin Terraform, please remove the \"connection-pooling-\" prefix and use\nunderscores instead of dashes in the name. For example,\n\"connection-pooling-pool-mode\" would be \"pool_mode\"."]
    pub fn set_flags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.flags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_ConnectionPoolConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_ConnectionPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_ConnectionPoolConfig {
    #[doc = "Whether to enabled Managed Connection Pool."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAlloydbInstance_ConnectionPoolConfig {
    pub fn build(self) -> AlloydbInstance_ConnectionPoolConfig {
        AlloydbInstance_ConnectionPoolConfig {
            enabled: self.enabled,
            flags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_ConnectionPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_ConnectionPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_ConnectionPoolConfigRef {
        AlloydbInstance_ConnectionPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_ConnectionPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to enabled Managed Connection Pool."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flags` after provisioning.\nFlags for configuring managed connection pooling when it is enabled.\nThese flags will only be set if 'connection_pool_config.enabled' is\ntrue.\nPlease see\nhttps://cloud.google.com/alloydb/docs/configure-managed-connection-pooling#configuration-options\nfor a comprehensive list of flags that can be set. To specify the flags\nin Terraform, please remove the \"connection-pooling-\" prefix and use\nunderscores instead of dashes in the name. For example,\n\"connection-pooling-pool-mode\" would be \"pool_mode\"."]
    pub fn flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pooler_count` after provisioning.\nThe number of running poolers per instance."]
    pub fn pooler_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pooler_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_MachineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_MachineConfig {
    #[doc =
        "Set the field `cpu_count`.\nThe number of CPUs in the VM instance. For read pool instances, this\nvalue is applied to the instances in the pool and is not replaced by\na fixed default."]
    pub fn set_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nMachine type of the VM instance.\nE.g. \"n2-highmem-4\", \"n2-highmem-8\", \"c4a-highmem-4-lssd\".\n'cpu_count' must match the number of vCPUs in the machine type."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_MachineConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_MachineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_MachineConfig {}

impl BuildAlloydbInstance_MachineConfig {
    pub fn build(self) -> AlloydbInstance_MachineConfig {
        AlloydbInstance_MachineConfig {
            cpu_count: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_MachineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_MachineConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_MachineConfigRef {
        AlloydbInstance_MachineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_MachineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of CPUs in the VM instance. For read pool instances, this\nvalue is applied to the instances in the pool and is not replaced by\na fixed default."]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of the VM instance.\nE.g. \"n2-highmem-4\", \"n2-highmem-8\", \"c4a-highmem-4-lssd\".\n'cpu_count' must match the number of vCPUs in the machine type."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    #[doc = "Set the field `cidr_range`.\nCIDR range for one authorized network of the instance."]
    pub fn set_cidr_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    type O = terrars::BlockAssignable<AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {}

impl BuildAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    pub fn build(self) -> AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
        AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks { cidr_range: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
        AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cidr_range` after provisioning.\nCIDR range for one authorized network of the instance."]
    pub fn cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbInstance_NetworkConfigDynamic {
    authorized_external_networks: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>>,
}

#[derive(Serialize)]
pub struct AlloydbInstance_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_outbound_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_external_networks: std
    ::option
    ::Option<std::vec::Vec<AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>>,
    dynamic: AlloydbInstance_NetworkConfigDynamic,
}

impl AlloydbInstance_NetworkConfig {
    #[doc =
        "Set the field `allocated_ip_range_override`.\nName of the allocated IP range for the private IP AlloyDB instance, for example: \"google-managed-services-default\".\nIf set, the instance IPs will be created from this allocated range and will override the IP range used by the parent cluster.\nThe range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range_override(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_outbound_public_ip`.\nEnabling outbound public ip for the instance."]
    pub fn set_enable_outbound_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_outbound_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_public_ip`.\nEnabling public ip for the instance. If a user wishes to disable this,\nplease also clear the list of the authorized external networks set on\nthe same instance."]
    pub fn set_enable_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorized_external_networks`.\n"]
    pub fn set_authorized_external_networks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorized_external_networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_external_networks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_NetworkConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_NetworkConfig {}

impl BuildAlloydbInstance_NetworkConfig {
    pub fn build(self) -> AlloydbInstance_NetworkConfig {
        AlloydbInstance_NetworkConfig {
            allocated_ip_range_override: core::default::Default::default(),
            enable_outbound_public_ip: core::default::Default::default(),
            enable_public_ip: core::default::Default::default(),
            authorized_external_networks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_NetworkConfigRef {
        AlloydbInstance_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_ip_range_override` after provisioning.\nName of the allocated IP range for the private IP AlloyDB instance, for example: \"google-managed-services-default\".\nIf set, the instance IPs will be created from this allocated range and will override the IP range used by the parent cluster.\nThe range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range_override", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_outbound_public_ip` after provisioning.\nEnabling outbound public ip for the instance."]
    pub fn enable_outbound_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_outbound_public_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_ip` after provisioning.\nEnabling public ip for the instance. If a user wishes to disable this,\nplease also clear the list of the authorized external networks set on\nthe same instance."]
    pub fn enable_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `authorized_external_networks` after provisioning.\n"]
    pub fn authorized_external_networks(
        &self,
    ) -> terrars::ListRef<AlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_external_networks", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_PscInstanceConfig_PscAutoConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_PscInstanceConfig_PscAutoConnections {
    #[doc =
        "Set the field `consumer_network`.\nThe consumer network for the PSC service automation, example:\n\"projects/vpc-host-project/global/networks/default\".\nThe consumer network might be hosted a different project than the\nconsumer project. The API expects the consumer project specified to be\nthe project ID (and not the project number)"]
    pub fn set_consumer_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consumer_project`.\nThe consumer project to which the PSC service automation endpoint will\nbe created. The API expects the consumer project to be the project ID(\nand not the project number)."]
    pub fn set_consumer_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_PscInstanceConfig_PscAutoConnections {
    type O = terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_PscInstanceConfig_PscAutoConnections {}

impl BuildAlloydbInstance_PscInstanceConfig_PscAutoConnections {
    pub fn build(self) -> AlloydbInstance_PscInstanceConfig_PscAutoConnections {
        AlloydbInstance_PscInstanceConfig_PscAutoConnections {
            consumer_network: core::default::Default::default(),
            consumer_project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
        AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nThe consumer network for the PSC service automation, example:\n\"projects/vpc-host-project/global/networks/default\".\nThe consumer network might be hosted a different project than the\nconsumer project. The API expects the consumer project specified to be\nthe project ID (and not the project number)"]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_network_status` after provisioning.\nThe status of the service connection policy."]
    pub fn consumer_network_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_project` after provisioning.\nThe consumer project to which the PSC service automation endpoint will\nbe created. The API expects the consumer project to be the project ID(\nand not the project number)."]
    pub fn consumer_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_project", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the PSC service automation endpoint."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the PSC service automation connection."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment_resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    #[doc =
        "Set the field `network_attachment_resource`.\nThe network attachment resource created in the consumer project to which the PSC interface will be linked.\nThis is of the format: \"projects/${CONSUMER_PROJECT}/regions/${REGION}/networkAttachments/${NETWORK_ATTACHMENT_NAME}\".\nThe network attachment must be in the same region as the instance."]
    pub fn set_network_attachment_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    type O = terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {}

impl BuildAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    pub fn build(self) -> AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
        AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
            network_attachment_resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
        AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment_resource` after provisioning.\nThe network attachment resource created in the consumer project to which the PSC interface will be linked.\nThis is of the format: \"projects/${CONSUMER_PROJECT}/regions/${REGION}/networkAttachments/${NETWORK_ATTACHMENT_NAME}\".\nThe network attachment must be in the same region as the instance."]
    pub fn network_attachment_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AlloydbInstance_PscInstanceConfigDynamic {
    psc_auto_connections: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbInstance_PscInstanceConfig_PscAutoConnections>>,
    psc_interface_configs: std
    ::option
    ::Option<terrars::DynamicBlock<AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>>,
}

#[derive(Serialize)]
pub struct AlloydbInstance_PscInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connections: std::option::Option<std::vec::Vec<AlloydbInstance_PscInstanceConfig_PscAutoConnections>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_configs: std::option::Option<std::vec::Vec<AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>>,
    dynamic: AlloydbInstance_PscInstanceConfigDynamic,
}

impl AlloydbInstance_PscInstanceConfig {
    #[doc =
        "Set the field `allowed_consumer_projects`.\nList of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.\nThese should be specified as project numbers only."]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig_PscAutoConnections>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_auto_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_auto_connections = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_interface_configs`.\n"]
    pub fn set_psc_interface_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_interface_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_interface_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_PscInstanceConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_PscInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_PscInstanceConfig {}

impl BuildAlloydbInstance_PscInstanceConfig {
    pub fn build(self) -> AlloydbInstance_PscInstanceConfig {
        AlloydbInstance_PscInstanceConfig {
            allowed_consumer_projects: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            psc_interface_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_PscInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_PscInstanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_PscInstanceConfigRef {
        AlloydbInstance_PscInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_PscInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\nList of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.\nThese should be specified as project numbers only."]
    pub fn allowed_consumer_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_dns_name` after provisioning.\nThe DNS name of the instance for PSC connectivity.\nName convention: <uid>.<uid>.<region>.alloydb-psc.goog"]
    pub fn psc_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_dns_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_attachment_link` after provisioning.\nThe service attachment created when Private Service Connect (PSC) is enabled for the instance.\nThe name of the resource will be in the format of\n'projects/<alloydb-tenant-project-number>/regions/<region-name>/serviceAttachments/<service-attachment-name>'"]
    pub fn service_attachment_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment_link", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<AlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_interface_configs` after provisioning.\n"]
    pub fn psc_interface_configs(&self) -> terrars::ListRef<AlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_QueryInsightsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_plans_per_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_application_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_client_address: std::option::Option<terrars::PrimField<bool>>,
}

impl AlloydbInstance_QueryInsightsConfig {
    #[doc =
        "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_length`.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn set_query_string_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_string_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_application_tags`.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn set_record_application_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_application_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_client_address`.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn set_record_client_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_client_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_QueryInsightsConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_QueryInsightsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_QueryInsightsConfig {}

impl BuildAlloydbInstance_QueryInsightsConfig {
    pub fn build(self) -> AlloydbInstance_QueryInsightsConfig {
        AlloydbInstance_QueryInsightsConfig {
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_QueryInsightsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_QueryInsightsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_QueryInsightsConfigRef {
        AlloydbInstance_QueryInsightsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_QueryInsightsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_length` after provisioning.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn query_string_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_application_tags` after provisioning.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn record_application_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_client_address` after provisioning.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn record_client_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_ReadPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl AlloydbInstance_ReadPoolConfig {
    #[doc = "Set the field `node_count`.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AlloydbInstance_ReadPoolConfig {
    type O = terrars::BlockAssignable<AlloydbInstance_ReadPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_ReadPoolConfig {}

impl BuildAlloydbInstance_ReadPoolConfig {
    pub fn build(self) -> AlloydbInstance_ReadPoolConfig {
        AlloydbInstance_ReadPoolConfig { node_count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_ReadPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_ReadPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_ReadPoolConfigRef {
        AlloydbInstance_ReadPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_ReadPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct AlloydbInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AlloydbInstance_Timeouts {
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

impl terrars::ToListMappable for AlloydbInstance_Timeouts {
    type O = terrars::BlockAssignable<AlloydbInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAlloydbInstance_Timeouts {}

impl BuildAlloydbInstance_Timeouts {
    pub fn build(self) -> AlloydbInstance_Timeouts {
        AlloydbInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AlloydbInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AlloydbInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AlloydbInstance_TimeoutsRef {
        AlloydbInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AlloydbInstance_TimeoutsRef {
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
