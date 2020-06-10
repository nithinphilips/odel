use super::tririga::ArrayOfLong;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};
use super::transport::Validate;
use xsd_types::types as xs;

//use   http://ws.tririga.com;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfDisplayLabel")]
pub struct ArrayOfDisplayLabel {
    #[yaserde(prefix = "dto", rename = "DisplayLabel")]
    pub display_label: Vec<DisplayLabel>,
}

impl Validate for ArrayOfDisplayLabel {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfChildBaseObjectType")]
pub struct ArrayOfChildBaseObjectType {
    #[yaserde(prefix = "dto", rename = "ChildBaseObjectType")]
    pub child_base_object_type: Vec<ChildBaseObjectType>,
}

impl Validate for ArrayOfChildBaseObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfChildRecord")]
pub struct ArrayOfChildRecord {
    #[yaserde(prefix = "dto", rename = "ChildRecord")]
    pub child_record: Vec<ChildRecord>,
}

impl Validate for ArrayOfChildRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "DisplayLabel")]
pub struct DisplayLabel {
    #[yaserde(prefix = "dto", rename = "fieldName")]
    pub field_name: String,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "sectionName")]
    pub section_name: String,
}

impl Validate for DisplayLabel {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfMetricQueryResponseHelper")]
pub struct ArrayOfMetricQueryResponseHelper {
    #[yaserde(prefix = "dto", rename = "MetricQueryResponseHelper")]
    pub metric_query_response_helper: Vec<MetricQueryResponseHelper>,
}

impl Validate for ArrayOfMetricQueryResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "FinancialToken")]
pub struct FinancialToken {
    #[yaserde(prefix = "dto", rename = "dateType")]
    pub date_type: Option<i32>,

    #[yaserde(prefix = "dto", rename = "endDate")]
    pub end_date: Option<String>,

    #[yaserde(prefix = "dto", rename = "groupId")]
    pub group_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "startDate")]
    pub start_date: Option<String>,

    #[yaserde(prefix = "dto", rename = "transactionType")]
    pub transaction_type: Option<String>,

    #[yaserde(prefix = "dto", rename = "unitOfMeasure")]
    pub unit_of_measure: Option<String>,

    #[yaserde(prefix = "dto", rename = "unitOfMeasureType")]
    pub unit_of_measure_type: Option<i64>,
}

impl Validate for FinancialToken {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ContinuationToken")]
pub struct ContinuationToken {
    #[yaserde(prefix = "dto", rename = "tokenString")]
    pub token_string: String,
}

impl Validate for ContinuationToken {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfAssociationFilter")]
pub struct ArrayOfAssociationFilter {
    #[yaserde(prefix = "dto", rename = "AssociationFilter")]
    pub association_filter: Vec<AssociationFilter>,
}

impl Validate for ArrayOfAssociationFilter {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryResponseHelper")]
pub struct QueryResponseHelper {
    #[yaserde(prefix = "dto", rename = "assocBoId")]
    pub assoc_bo_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "assocId")]
    pub assoc_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "queryResponseColumns")]
    pub query_response_columns: Option<ArrayOfQueryResponseColumn>,

    // #[yaserde(flatten)]
    // pub base: AbstractQueryResponseHelper,

    #[yaserde(prefix = "dto", rename = "boId")]
    pub bo_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,
}

impl Validate for QueryResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfAssociationDefinition")]
pub struct ArrayOfAssociationDefinition {
    #[yaserde(prefix = "dto", rename = "AssociationDefinition")]
    pub association_definition: Vec<AssociationDefinition>,
}

impl Validate for ArrayOfAssociationDefinition {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfListType")]
pub struct ArrayOfListType {
    #[yaserde(prefix = "dto", rename = "ListType")]
    pub list_type: Vec<ListType>,
}

impl Validate for ArrayOfListType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "RecordField")]
pub struct RecordField {
    pub base: Field,
}

impl Validate for RecordField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AvailableAction")]
pub struct AvailableAction {
    #[yaserde(prefix = "dto", rename = "availableTransitions")]
    pub available_transitions: Option<ArrayOfTransition>,

    #[yaserde(prefix = "dto", rename = "currentState")]
    pub current_state: Option<String>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,
}

impl Validate for AvailableAction {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfTransition")]
pub struct ArrayOfTransition {
    #[yaserde(prefix = "dto", rename = "Transition")]
    pub transition: Vec<Transition>,
}

impl Validate for ArrayOfTransition {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ApplicationInfo")]
pub struct ApplicationInfo {
    #[yaserde(prefix = "dto", rename = "apiVersion")]
    pub api_version: Option<String>,

    #[yaserde(prefix = "dto", rename = "dbBuildNumber")]
    pub db_build_number: Option<String>,

    #[yaserde(prefix = "dto", rename = "tririgaBuildNumber")]
    pub tririga_build_number: Option<String>,
}

impl Validate for ApplicationInfo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ResponseHelper")]
pub struct ResponseHelper {
    #[yaserde(prefix = "dto", rename = "key")]
    pub key: Option<String>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "status")]
    pub status: Option<String>,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: Option<String>,
}

impl Validate for ResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfIntegrationRecord")]
pub struct ArrayOfIntegrationRecord {
    #[yaserde(prefix = "dto", rename = "IntegrationRecord")]
    pub integration_record: Vec<IntegrationRecord>,
}

impl Validate for ArrayOfIntegrationRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfQueryResponseColumn")]
pub struct ArrayOfQueryResponseColumn {
    #[yaserde(prefix = "dto", rename = "QueryResponseColumn")]
    pub query_response_column: Vec<QueryResponseColumn>,
}

impl Validate for ArrayOfQueryResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "GroupBy")]
pub struct GroupBy {
    pub base: AbstractMetricQueryCriteria,
}

impl Validate for GroupBy {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryResponseColumn")]
pub struct QueryResponseColumn {
    #[yaserde(prefix = "dto", rename = "displayValue")]
    pub display_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "index")]
    pub index: Option<i32>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "section")]
    pub section: Option<String>,

    #[yaserde(prefix = "dto", rename = "uom")]
    pub uom: Option<String>,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: Option<String>,
}

impl Validate for QueryResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Module")]
pub struct Module {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for Module {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ConversionGroup")]
pub struct ConversionGroup {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for ConversionGroup {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfFilter")]
pub struct ArrayOfFilter {
    #[yaserde(prefix = "dto", rename = "Filter")]
    pub filter: Vec<Filter>,
}

impl Validate for ArrayOfFilter {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfConversionGroup")]
pub struct ArrayOfConversionGroup {
    #[yaserde(prefix = "dto", rename = "ConversionGroup")]
    pub conversion_group: Vec<ConversionGroup>,
}

impl Validate for ArrayOfConversionGroup {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "MetricQueryResult")]
pub struct MetricQueryResult {
    #[yaserde(prefix = "dto", rename = "metricQueryResponseHelpers")]
    pub metric_query_response_helpers: Option<ArrayOfMetricQueryResponseHelper>,

    pub base: AbstractQueryResult,
}

impl Validate for MetricQueryResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfField")]
pub struct ArrayOfField {
    #[yaserde(prefix = "dto", rename = "Field")]
    pub field: Vec<Field>,
}

impl Validate for ArrayOfField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfCurrencyConversionRate")]
pub struct ArrayOfCurrencyConversionRate {
    #[yaserde(prefix = "dto", rename = "CurrencyConversionRate")]
    pub currency_conversion_rate: Vec<CurrencyConversionRate>,
}

impl Validate for ArrayOfCurrencyConversionRate {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ObjectType")]
pub struct ObjectType {
    #[yaserde(prefix = "dto", rename = "approvalHistory")]
    pub approval_history: Option<bool>,

    #[yaserde(prefix = "dto", rename = "auditAccess")]
    pub audit_access: Option<bool>,

    #[yaserde(prefix = "dto", rename = "auditActions")]
    pub audit_actions: Option<bool>,

    #[yaserde(prefix = "dto", rename = "auditDataChanges")]
    pub audit_data_changes: Option<bool>,

    #[yaserde(prefix = "dto", rename = "createdById")]
    pub created_by_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "description")]
    pub description: Option<String>,

    #[yaserde(prefix = "dto", rename = "displayName")]
    pub display_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "hasCalendar")]
    pub has_calendar: Option<bool>,

    #[yaserde(prefix = "dto", rename = "moduleId")]
    pub module_id: Option<i32>,

    #[yaserde(prefix = "dto", rename = "requireExplanation")]
    pub require_explanation: Option<bool>,

    #[yaserde(prefix = "dto", rename = "sections")]
    pub sections: Option<ArrayOfSection>,

    #[yaserde(prefix = "dto", rename = "showSingleTab")]
    pub show_single_tab: Option<bool>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<i32>,

    pub base: HierarchyObjectType,
}

impl Validate for ObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ResponseHelperHeader")]
pub struct ResponseHelperHeader {
    #[yaserde(prefix = "dto", rename = "anyFailed")]
    pub any_failed: Option<bool>,

    #[yaserde(prefix = "dto", rename = "failed")]
    pub failed: Option<i32>,

    #[yaserde(prefix = "dto", rename = "responseHelpers")]
    pub response_helpers: Option<ArrayOfResponseHelper>,

    #[yaserde(prefix = "dto", rename = "successful")]
    pub successful: Option<i32>,

    #[yaserde(prefix = "dto", rename = "total")]
    pub total: Option<i32>,
}

impl Validate for ResponseHelperHeader {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AssociationRecord")]
pub struct AssociationRecord {
    #[yaserde(prefix = "dto", rename = "associations")]
    pub associations: Option<ArrayOfAssociation>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,
}

impl Validate for AssociationRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "IntegrationField")]
pub struct IntegrationField {
    #[yaserde(prefix = "dto", rename = "name")]
    pub name: String,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: String,
}

impl Validate for IntegrationField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfObjectTypeActionStep")]
pub struct ArrayOfObjectTypeActionStep {
    #[yaserde(prefix = "dto", rename = "ObjectTypeActionStep")]
    pub object_type_action_step: Vec<ObjectTypeActionStep>,
}

impl Validate for ArrayOfObjectTypeActionStep {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfMetricQueryFilter")]
pub struct ArrayOfMetricQueryFilter {
    #[yaserde(prefix = "dto", rename = "MetricQueryFilter")]
    pub metric_query_filter: Vec<MetricQueryFilter>,
}

impl Validate for ArrayOfMetricQueryFilter {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "FieldSortOrder")]
pub struct FieldSortOrder {
    #[yaserde(prefix = "dto", rename = "dataType")]
    pub data_type: i32,

    #[yaserde(prefix = "dto", rename = "fieldLabel")]
    pub field_label: Option<String>,

    #[yaserde(prefix = "dto", rename = "fieldName")]
    pub field_name: String,

    #[yaserde(prefix = "dto", rename = "sectionName")]
    pub section_name: String,
}

impl Validate for FieldSortOrder {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AbstractQueryResponseHelper")]
pub struct AbstractQueryResponseHelper {
    #[yaserde(prefix = "dto", rename = "boId")]
    pub bo_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<String>,
}

impl Validate for AbstractQueryResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ChildBaseObjectType")]
pub struct ChildBaseObjectType {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for ChildBaseObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfTriggerActions")]
pub struct ArrayOfTriggerActions {
    #[yaserde(prefix = "dto", rename = "TriggerActions")]
    pub trigger_actions: Vec<TriggerActions>,
}

impl Validate for ArrayOfTriggerActions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Field")]
pub struct Field {
    #[yaserde(prefix = "dto", rename = "associationString")]
    pub association_string: Option<String>,

    #[yaserde(prefix = "dto", rename = "attributeSequence")]
    pub attribute_sequence: Option<i64>,

    #[yaserde(prefix = "dto", rename = "autoPopulate")]
    pub auto_populate: Option<bool>,

    #[yaserde(prefix = "dto", rename = "colSpan")]
    pub col_span: Option<i32>,

    #[yaserde(prefix = "dto", rename = "columnSeq")]
    pub column_seq: Option<i64>,

    #[yaserde(prefix = "dto", rename = "dataType")]
    pub data_type: Option<DataType>,

    #[yaserde(prefix = "dto", rename = "defaultValue")]
    pub default_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "dependent")]
    pub dependent: Option<bool>,

    #[yaserde(prefix = "dto", rename = "displayValue")]
    pub display_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "editUOMList")]
    pub edit_uom_list: Option<bool>,

    #[yaserde(prefix = "dto", rename = "endRow")]
    pub end_row: Option<i32>,

    #[yaserde(prefix = "dto", rename = "fieldId")]
    pub field_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "financialToken")]
    pub financial_token: Option<FinancialToken>,

    #[yaserde(prefix = "dto", rename = "formula")]
    pub formula: Option<Formula>,

    #[yaserde(prefix = "dto", rename = "fullPath")]
    pub full_path: Option<bool>,

    #[yaserde(prefix = "dto", rename = "generateOnCreate")]
    pub generate_on_create: Option<bool>,

    #[yaserde(prefix = "dto", rename = "guiFieldId")]
    pub gui_field_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiFieldName")]
    pub gui_field_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "guiFieldType")]
    pub gui_field_type: Option<i32>,

    #[yaserde(prefix = "dto", rename = "guiId")]
    pub gui_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiRootClassificationId")]
    pub gui_root_classification_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiRootClassificationName")]
    pub gui_root_classification_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "guiSectionId")]
    pub gui_section_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiSectionName")]
    pub gui_section_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "guiTabId")]
    pub gui_tab_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "hierarchyObjectId")]
    pub hierarchy_object_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "listId")]
    pub list_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "listModuleId")]
    pub list_module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "liveLink")]
    pub live_link: Option<bool>,

    #[yaserde(prefix = "dto", rename = "locatorField")]
    pub locator_field: Option<bool>,

    #[yaserde(prefix = "dto", rename = "locatorModuleId")]
    pub locator_module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "mobileField")]
    pub mobile_field: Option<bool>,

    #[yaserde(prefix = "dto", rename = "mobileFieldSeq")]
    pub mobile_field_seq: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "precision")]
    pub precision: Option<i64>,

    #[yaserde(prefix = "dto", rename = "queryId")]
    pub query_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "queryName")]
    pub query_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "readOnly")]
    pub read_only: Option<bool>,

    #[yaserde(prefix = "dto", rename = "required")]
    pub required: Option<bool>,

    #[yaserde(prefix = "dto", rename = "resultColumn")]
    pub result_column: Option<bool>,

    #[yaserde(prefix = "dto", rename = "rollupSource")]
    pub rollup_source: Option<String>,

    #[yaserde(prefix = "dto", rename = "rootClassification")]
    pub root_classification: Option<String>,

    #[yaserde(prefix = "dto", rename = "rootClassificationId")]
    pub root_classification_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sectionName")]
    pub section_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "seq")]
    pub seq: Option<i64>,

    #[yaserde(prefix = "dto", rename = "specId")]
    pub spec_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "specTemplateId")]
    pub spec_template_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "stagingTableField")]
    pub staging_table_field: Option<bool>,

    #[yaserde(prefix = "dto", rename = "startRow")]
    pub start_row: Option<i32>,

    #[yaserde(prefix = "dto", rename = "subAttributeType")]
    pub sub_attribute_type: Option<String>,

    #[yaserde(prefix = "dto", rename = "subCategoryId")]
    pub sub_category_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sumThisField")]
    pub sum_this_field: Option<bool>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<String>,

    #[yaserde(prefix = "dto", rename = "unitOfMeasure")]
    pub unit_of_measure: Option<String>,

    #[yaserde(prefix = "dto", rename = "unitOfMeasureSourceAttribute")]
    pub unit_of_measure_source_attribute: Option<i64>,

    #[yaserde(prefix = "dto", rename = "unitOfMeasureType")]
    pub unit_of_measure_type: Option<i64>,

    #[yaserde(prefix = "dto", rename = "validation")]
    pub validation: Option<String>,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: Option<String>,

    #[yaserde(prefix = "dto", rename = "valueId")]
    pub value_id: Option<i64>,
}

impl Validate for Field {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AbstractMetricQueryCriteria")]
pub struct AbstractMetricQueryCriteria {
    #[yaserde(prefix = "dto", rename = "fieldName")]
    pub field_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: Option<String>,
}

impl Validate for AbstractMetricQueryCriteria {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Tab")]
pub struct Tab {
    #[yaserde(prefix = "dto", rename = "sections")]
    pub sections: Option<ArrayOfSection>,

    #[yaserde(prefix = "dto", rename = "tabId")]
    pub tab_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "tabName")]
    pub tab_name: Option<String>,
}

impl Validate for Tab {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfIntegrationSection-1-")]
pub struct ArrayOfIntegrationSection1 {
    #[yaserde(prefix = "dto", rename = "IntegrationSection")]
    pub integration_section: Vec<IntegrationSection>,
}

impl Validate for ArrayOfIntegrationSection1 {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "MetricQueryFilter")]
pub struct MetricQueryFilter {
    #[yaserde(prefix = "dto", rename = "operator")]
    pub operator: Option<i32>,

    pub base: AbstractMetricQueryCriteria,
}

impl Validate for MetricQueryFilter {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "RegularFormula")]
pub struct RegularFormula {}

impl Validate for RegularFormula {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "FieldKey")]
pub struct FieldKey {
    #[yaserde(prefix = "dto", rename = "fieldName")]
    pub field_name: String,

    #[yaserde(prefix = "dto", rename = "sectionName")]
    pub section_name: String,
}

impl Validate for FieldKey {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfExtendedFormulaParam")]
pub struct ArrayOfExtendedFormulaParam {
    #[yaserde(prefix = "dto", rename = "ExtendedFormulaParam")]
    pub extended_formula_param: Vec<ExtendedFormulaParam>,
}

impl Validate for ArrayOfExtendedFormulaParam {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfKeywordSearchResult")]
pub struct ArrayOfKeywordSearchResult {
    #[yaserde(prefix = "dto", rename = "KeywordSearchResult")]
    pub keyword_search_result: Vec<KeywordSearchResult>,
}

impl Validate for ArrayOfKeywordSearchResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "IntegrationRecord")]
pub struct IntegrationRecord {
    #[yaserde(prefix = "dto", rename = "actionName")]
    pub action_name: String,

    #[yaserde(prefix = "dto", rename = "guiId")]
    pub gui_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: i64,

    #[yaserde(prefix = "dto", rename = "key")]
    pub key: Option<String>,

    #[yaserde(prefix = "dto", rename = "moduleId")]
    pub module_id: i32,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectPath")]
    pub object_path: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectTypeId")]
    pub object_type_id: i64,

    #[yaserde(prefix = "dto", rename = "objectTypeName")]
    pub object_type_name: String,

    #[yaserde(prefix = "dto", rename = "parentId")]
    pub parent_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "projectId")]
    pub project_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sections")]
    pub sections: ArrayOfIntegrationSection1,
}

impl Validate for IntegrationRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfSubAction")]
pub struct ArrayOfSubAction {
    #[yaserde(prefix = "dto", rename = "SubAction")]
    pub sub_action: Vec<SubAction>,
}

impl Validate for ArrayOfSubAction {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Association")]
pub struct Association {
    #[yaserde(prefix = "dto", rename = "associatedRecordId")]
    pub associated_record_id: i64,

    #[yaserde(prefix = "dto", rename = "associatedRecordName")]
    pub associated_record_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "associationName")]
    pub association_name: String,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: i64,

    #[yaserde(prefix = "dto", rename = "reverseAssociationName")]
    pub reverse_association_name: Option<String>,

    pub base: AssociationDefinition,
}

impl Validate for Association {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "HttpSession")]
pub struct HttpSession {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "token")]
    pub token: Option<String>,
}

impl Validate for HttpSession {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfQueryMultiBoResponseColumn")]
pub struct ArrayOfQueryMultiBoResponseColumn {
    #[yaserde(prefix = "dto", rename = "QueryMultiBoResponseColumn")]
    pub query_multi_bo_response_column: Vec<QueryMultiBoResponseColumn>,
}

impl Validate for ArrayOfQueryMultiBoResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Currency")]
pub struct Currency {
    #[yaserde(prefix = "dto", rename = "abbreviation")]
    pub abbreviation: Option<String>,

    #[yaserde(prefix = "dto", rename = "decimal")]
    pub decimal: Option<String>,

    #[yaserde(prefix = "dto", rename = "delimiter")]
    pub delimiter: Option<String>,

    #[yaserde(prefix = "dto", rename = "format")]
    pub format: Option<String>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "symbol")]
    pub symbol: Option<String>,
}

impl Validate for Currency {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfTab")]
pub struct ArrayOfTab {
    #[yaserde(prefix = "dto", rename = "Tab")]
    pub tab: Vec<Tab>,
}

impl Validate for ArrayOfTab {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Row")]
pub struct Row {
    #[yaserde(prefix = "dto", rename = "fields")]
    pub fields: Option<ArrayOfField>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,
}

impl Validate for Row {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfState")]
pub struct ArrayOfState {
    #[yaserde(prefix = "dto", rename = "State")]
    pub state: Vec<State>,
}

impl Validate for ArrayOfState {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryMultiBoResponseColumn")]
pub struct QueryMultiBoResponseColumn {
    #[yaserde(prefix = "dto", rename = "bo")]
    pub bo: Option<String>,

    #[yaserde(prefix = "dto", rename = "module")]
    pub module: Option<String>,

    #[yaserde(prefix = "dto", rename = "multiBoFieldIndex")]
    pub multi_bo_field_index: Option<String>,

    pub base: QueryResponseColumn,
}

impl Validate for QueryMultiBoResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfIntegrationRows")]
pub struct ArrayOfIntegrationRows {
    #[yaserde(prefix = "dto", rename = "IntegrationRows")]
    pub integration_rows: Vec<IntegrationRows>,
}

impl Validate for ArrayOfIntegrationRows {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryResult")]
pub struct QueryResult {
    #[yaserde(prefix = "dto", rename = "queryResponseHelpers")]
    pub query_response_helpers: Option<ArrayOfQueryResponseHelper>,

    pub base: AbstractQueryResult,
}

impl Validate for QueryResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ExtendedFormula")]
pub struct ExtendedFormula {
    #[yaserde(prefix = "dto", rename = "expression")]
    pub expression: Option<String>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "parameters")]
    pub parameters: Option<ArrayOfExtendedFormulaParam>,
}

impl Validate for ExtendedFormula {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryMultiBoResponseHelper")]
pub struct QueryMultiBoResponseHelper {
    #[yaserde(prefix = "dto", rename = "queryMultiBoResponseColumns")]
    pub query_multi_bo_response_columns: Option<ArrayOfQueryMultiBoResponseColumn>,

    pub base: AbstractQueryResponseHelper,
}

impl Validate for QueryMultiBoResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfListItem")]
pub struct ArrayOfListItem {
    #[yaserde(prefix = "dto", rename = "ListItem")]
    pub list_item: Vec<ListItem>,
}

impl Validate for ArrayOfListItem {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfAssociation")]
pub struct ArrayOfAssociation {
    #[yaserde(prefix = "dto", rename = "Association")]
    pub association: Vec<Association>,
}

impl Validate for ArrayOfAssociation {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfResponseHelper")]
pub struct ArrayOfResponseHelper {
    #[yaserde(prefix = "dto", rename = "ResponseHelper")]
    pub response_helper: Vec<ResponseHelper>,
}

impl Validate for ArrayOfResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfAvailableAction")]
pub struct ArrayOfAvailableAction {
    #[yaserde(prefix = "dto", rename = "AvailableAction")]
    pub available_action: Vec<AvailableAction>,
}

impl Validate for ArrayOfAvailableAction {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AssociationDefinition")]
pub struct AssociationDefinition {
    #[yaserde(prefix = "dto", rename = "associatedModuleId")]
    pub associated_module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "associatedObjectTypeId")]
    pub associated_object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "associationName")]
    pub association_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "inverseAssociationName")]
    pub inverse_association_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "sourceModuleId")]
    pub source_module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sourceObjectTypeId")]
    pub source_object_type_id: Option<i64>,
}

impl Validate for AssociationDefinition {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfMetricQueryResponseColumn")]
pub struct ArrayOfMetricQueryResponseColumn {
    #[yaserde(prefix = "dto", rename = "MetricQueryResponseColumn")]
    pub metric_query_response_column: Vec<MetricQueryResponseColumn>,
}

impl Validate for ArrayOfMetricQueryResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "RecordSection")]
pub struct RecordSection {
    #[yaserde(prefix = "dto", rename = "recordFields")]
    pub record_fields: Option<ArrayOfRecordField>,

    pub base: Section,
}

impl Validate for RecordSection {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Formula")]
pub struct Formula {
    #[yaserde(prefix = "dto", rename = "extendedFormula")]
    pub extended_formula: Vec<ExtendedFormula>,

    #[yaserde(prefix = "dto", rename = "regularFormula")]
    pub regular_formula: Vec<RegularFormula>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<i32>,
}

impl Validate for Formula {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "State")]
pub struct State {
    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "state")]
    pub state: Option<String>,

    #[yaserde(prefix = "dto", rename = "trans")]
    pub trans: Option<ArrayOfTransition>,
}

impl Validate for State {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "IntegrationSection")]
pub struct IntegrationSection {
    #[yaserde(prefix = "dto", rename = "fields")]
    pub fields: Option<ArrayOfIntegrationField>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: String,

    #[yaserde(prefix = "dto", rename = "rows")]
    pub rows: Option<ArrayOfIntegrationRows>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<String>,
}

impl Validate for IntegrationSection {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfRecordField")]
pub struct ArrayOfRecordField {
    #[yaserde(prefix = "dto", rename = "RecordField")]
    pub record_field: Vec<RecordField>,
}

impl Validate for ArrayOfRecordField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfCurrency")]
pub struct ArrayOfCurrency {
    #[yaserde(prefix = "dto", rename = "Currency")]
    pub currency: Vec<Currency>,
}

impl Validate for ArrayOfCurrency {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "MetricQueryResponseHelper")]
pub struct MetricQueryResponseHelper {
    #[yaserde(prefix = "dto", rename = "metricQueryResponseColumns")]
    pub metric_query_response_columns: Option<ArrayOfMetricQueryResponseColumn>,

    #[yaserde(prefix = "dto", rename = "primaryGroupByDisplayValue")]
    pub primary_group_by_display_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "primaryGroupById")]
    pub primary_group_by_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "primaryGroupByLabel")]
    pub primary_group_by_label: Option<String>,

    #[yaserde(prefix = "dto", rename = "primaryGroupByName")]
    pub primary_group_by_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "primaryGroupByValue")]
    pub primary_group_by_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "secondaryGroupByDisplayValue")]
    pub secondary_group_by_display_value: Option<String>,

    #[yaserde(prefix = "dto", rename = "secondaryGroupById")]
    pub secondary_group_by_id: Option<String>,

    #[yaserde(prefix = "dto", rename = "secondaryGroupByLabel")]
    pub secondary_group_by_label: Option<String>,

    #[yaserde(prefix = "dto", rename = "secondaryGroupByName")]
    pub secondary_group_by_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "secondaryGroupByValue")]
    pub secondary_group_by_value: Option<String>,

    pub base: AbstractQueryResponseHelper,
}

impl Validate for MetricQueryResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AssociationFilter")]
pub struct AssociationFilter {
    #[yaserde(prefix = "dto", rename = "associationName")]
    pub association_name: String,

    #[yaserde(prefix = "dto", rename = "moduleName")]
    pub module_name: String,

    #[yaserde(prefix = "dto", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "reverseAssociation")]
    pub reverse_association: bool,

    #[yaserde(prefix = "dto", rename = "runTimeData")]
    pub run_time_data: String,
}

impl Validate for AssociationFilter {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "SubAction")]
pub struct SubAction {
    #[yaserde(prefix = "dto", rename = "action")]
    pub action: Option<String>,

    #[yaserde(prefix = "dto", rename = "actionId")]
    pub action_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,
}

impl Validate for SubAction {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ListItem")]
pub struct ListItem {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "languageId")]
    pub language_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sequenceNumber")]
    pub sequence_number: Option<i64>,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: Option<String>,
}

impl Validate for ListItem {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfSection")]
pub struct ArrayOfSection {
    #[yaserde(prefix = "dto", rename = "Section")]
    pub section: Vec<Section>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "CurrencyConversionRate")]
pub struct CurrencyConversionRate {
    #[yaserde(prefix = "dto", rename = "conversionGroup")]
    pub conversion_group: String,

    #[yaserde(prefix = "dto", rename = "conversionRate")]
    pub conversion_rate: f64,

    #[yaserde(prefix = "dto", rename = "endDate")]
    pub end_date: xs::DateTime,

    #[yaserde(prefix = "dto", rename = "fromCurrency")]
    pub from_currency: String,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "startDate")]
    pub start_date: String,

    #[yaserde(prefix = "dto", rename = "toCurrency")]
    pub to_currency: String,
}

impl Validate for CurrencyConversionRate {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfActionItem")]
pub struct ArrayOfActionItem {
    #[yaserde(prefix = "dto", rename = "ActionItem")]
    pub action_item: Vec<ActionItem>,
}

impl Validate for ArrayOfActionItem {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ExtendedFormulaParam")]
pub struct ExtendedFormulaParam {
    #[yaserde(prefix = "dto", rename = "associationChain")]
    pub association_chain: Option<ArrayOfAssociationDefinition>,

    #[yaserde(prefix = "dto", rename = "fieldKey")]
    pub field_key: Option<FieldKey>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for ExtendedFormulaParam {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ObjectTypeActionStep")]
pub struct ObjectTypeActionStep {
    #[yaserde(prefix = "dto", rename = "action")]
    pub action: Option<String>,

    #[yaserde(prefix = "dto", rename = "actionId")]
    pub action_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectTypeActionSteps")]
    pub object_type_action_steps: Vec<ArrayOfObjectTypeActionStep>,
}

impl Validate for ObjectTypeActionStep {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfBaseObjectType")]
pub struct ArrayOfBaseObjectType {
    #[yaserde(prefix = "dto", rename = "BaseObjectType")]
    pub base_object_type: Vec<BaseObjectType>,
}

impl Validate for ArrayOfBaseObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "QueryMultiBoResult")]
pub struct QueryMultiBoResult {
    #[yaserde(prefix = "dto", rename = "queryMultiBoResponseHelpers")]
    pub query_multi_bo_response_helpers: Option<ArrayOfQueryMultiBoResponseHelper>,

    pub base: AbstractQueryResult,
}

impl Validate for QueryMultiBoResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "TriggerActions")]
pub struct TriggerActions {
    #[yaserde(prefix = "dto", rename = "actionName")]
    pub action_name: String,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: i64,
}

impl Validate for TriggerActions {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ListType")]
pub struct ListType {
    #[yaserde(prefix = "dto", rename = "dependantList")]
    pub dependant_list: Option<bool>,

    #[yaserde(prefix = "dto", rename = "description")]
    pub description: Option<String>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "language")]
    pub language: Option<String>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "parentId")]
    pub parent_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "parentProduct")]
    pub parent_product: Option<String>,

    #[yaserde(prefix = "dto", rename = "product")]
    pub product: Option<String>,

    #[yaserde(prefix = "dto", rename = "sourceClassId")]
    pub source_class_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sourceObjectTypeId")]
    pub source_object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sourceType")]
    pub source_type: Option<i32>,

    #[yaserde(prefix = "dto", rename = "system")]
    pub system: Option<bool>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<String>,
}

impl Validate for ListType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfFieldSortOrder")]
pub struct ArrayOfFieldSortOrder {
    #[yaserde(prefix = "dto", rename = "FieldSortOrder")]
    pub field_sort_order: Vec<FieldSortOrder>,
}

impl Validate for ArrayOfFieldSortOrder {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "DataType")]
pub struct DataType {
    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<String>,

    #[yaserde(prefix = "dto", rename = "typeCode")]
    pub type_code: Option<i32>,
}

impl Validate for DataType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Filter")]
pub struct Filter {
    #[yaserde(prefix = "dto", rename = "dataType")]
    pub data_type: i32,

    #[yaserde(prefix = "dto", rename = "fieldName")]
    pub field_name: String,

    #[yaserde(prefix = "dto", rename = "operator")]
    pub operator: i32,

    #[yaserde(prefix = "dto", rename = "sectionName")]
    pub section_name: String,

    #[yaserde(prefix = "dto", rename = "value")]
    pub value: String,
}

impl Validate for Filter {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "KeywordSearchResult")]
pub struct KeywordSearchResult {
    #[yaserde(prefix = "dto", rename = "moduleId")]
    pub module_id: Option<i32>,

    #[yaserde(prefix = "dto", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectTypeId")]
    pub object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "recordIds")]
    pub record_ids: Option<ArrayOfLong>,
}

impl Validate for KeywordSearchResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "HierarchyObjectType")]
pub struct HierarchyObjectType {
    #[yaserde(prefix = "dto", rename = "children")]
    pub children: Vec<ArrayOfHierarchyObjectType>,

    #[yaserde(prefix = "dto", rename = "rootOfHierarchy")]
    pub root_of_hierarchy: Option<bool>,

    pub base: BaseObjectType,
}

impl Validate for HierarchyObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfHierarchyObjectType")]
pub struct ArrayOfHierarchyObjectType {
    #[yaserde(prefix = "dto", rename = "HierarchyObjectType")]
    pub hierarchy_object_type: Vec<HierarchyObjectType>,
}

impl Validate for ArrayOfHierarchyObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ActionItem")]
pub struct ActionItem {
    #[yaserde(prefix = "dto", rename = "taskId")]
    pub task_id: i64,

    #[yaserde(prefix = "dto", rename = "workflowId")]
    pub workflow_id: i64,
}

impl Validate for ActionItem {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfRow")]
pub struct ArrayOfRow {
    #[yaserde(prefix = "dto", rename = "Row")]
    pub row: Vec<Row>,
}

impl Validate for ArrayOfRow {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfRecord")]
pub struct ArrayOfRecord {
    #[yaserde(prefix = "dto", rename = "Record")]
    pub record: Vec<Record>,
}

impl Validate for ArrayOfRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Record")]
pub struct Record {
    #[yaserde(prefix = "dto", rename = "currentState")]
    pub current_state: Option<String>,

    #[yaserde(prefix = "dto", rename = "description")]
    pub description: Option<String>,

    #[yaserde(prefix = "dto", rename = "guiId")]
    pub gui_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiName")]
    pub gui_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "moduleId")]
    pub module_id: Option<i32>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectPath")]
    pub object_path: Option<String>,

    #[yaserde(prefix = "dto", rename = "objectTypeId")]
    pub object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "parentId")]
    pub parent_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "sections")]
    pub sections: Option<ArrayOfRecordSection>,

    #[yaserde(prefix = "dto", rename = "tabs")]
    pub tabs: Option<ArrayOfTab>,
}

impl Validate for Record {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "MetricQueryResponseColumn")]
pub struct MetricQueryResponseColumn {
    #[yaserde(prefix = "dto", rename = "aggregate")]
    pub aggregate: Option<String>,

    #[yaserde(prefix = "dto", rename = "thresholdRangeColor")]
    pub threshold_range_color: Option<String>,

    #[yaserde(prefix = "dto", rename = "thresholdRangeIconUrl")]
    pub threshold_range_icon_url: Option<String>,

    pub base: QueryResponseColumn,
}

impl Validate for MetricQueryResponseColumn {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfAssociationRecord")]
pub struct ArrayOfAssociationRecord {
    #[yaserde(prefix = "dto", rename = "AssociationRecord")]
    pub association_record: Vec<AssociationRecord>,
}

impl Validate for ArrayOfAssociationRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ObjectTypeAction")]
pub struct ObjectTypeAction {
    #[yaserde(prefix = "dto", rename = "boActionSteps")]
    pub bo_action_steps: Option<ArrayOfObjectTypeActionStep>,

    #[yaserde(prefix = "dto", rename = "guiId")]
    pub gui_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "moduleId")]
    pub module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "objectTypeId")]
    pub object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,
}

impl Validate for ObjectTypeAction {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "AbstractQueryResult")]
pub struct AbstractQueryResult {
    #[yaserde(prefix = "dto", rename = "continuationToken")]
    pub continuation_token: Option<ContinuationToken>,

    #[yaserde(prefix = "dto", rename = "totalResults")]
    pub total_results: Option<i32>,
}

impl Validate for AbstractQueryResult {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfModule")]
pub struct ArrayOfModule {
    #[yaserde(prefix = "dto", rename = "Module")]
    pub module: Vec<Module>,
}

impl Validate for ArrayOfModule {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Section")]
pub struct Section {
    #[yaserde(prefix = "dto", rename = "fields")]
    pub fields: Option<ArrayOfField>,

    #[yaserde(prefix = "dto", rename = "guiSectionId")]
    pub gui_section_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "guiSectionName")]
    pub gui_section_name: Option<String>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,

    #[yaserde(prefix = "dto", rename = "refModuleId")]
    pub ref_module_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "refObjectTypeId")]
    pub ref_object_type_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "rows")]
    pub rows: Option<ArrayOfRow>,

    #[yaserde(prefix = "dto", rename = "sections")]
    pub sections: Vec<ArrayOfSection>,

    #[yaserde(prefix = "dto", rename = "type")]
    pub _type: Option<String>,

    #[yaserde(prefix = "dto", rename = "typeId")]
    pub type_id: Option<i32>,
}

impl Validate for Section {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfIntegrationField")]
pub struct ArrayOfIntegrationField {
    #[yaserde(prefix = "dto", rename = "IntegrationField")]
    pub integration_field: Vec<IntegrationField>,
}

impl Validate for ArrayOfIntegrationField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Project")]
pub struct Project {
    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for Project {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfQueryMultiBoResponseHelper")]
pub struct ArrayOfQueryMultiBoResponseHelper {
    #[yaserde(prefix = "dto", rename = "QueryMultiBoResponseHelper")]
    pub query_multi_bo_response_helper: Vec<QueryMultiBoResponseHelper>,
}

impl Validate for ArrayOfQueryMultiBoResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "BaseObjectType")]
pub struct BaseObjectType {
    #[yaserde(prefix = "dto", rename = "childBaseObjectTypes")]
    pub child_base_object_types: Vec<ArrayOfChildBaseObjectType>,

    #[yaserde(prefix = "dto", rename = "id")]
    pub id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "name")]
    pub name: Option<String>,
}

impl Validate for BaseObjectType {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "IntegrationRows")]
pub struct IntegrationRows {
    #[yaserde(prefix = "dto", rename = "action")]
    pub action: String,

    #[yaserde(prefix = "dto", rename = "fields")]
    pub fields: Option<ArrayOfIntegrationField>,

    #[yaserde(prefix = "dto", rename = "recordId")]
    pub record_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "rowIndex")]
    pub row_index: Option<i32>,
}

impl Validate for IntegrationRows {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ChildRecord")]
pub struct ChildRecord {
    #[yaserde(prefix = "dto", rename = "childCount")]
    pub child_count: Option<i64>,

    #[yaserde(prefix = "dto", rename = "path")]
    pub path: Option<String>,

    pub base: Record,
}

impl Validate for ChildRecord {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "Transition")]
pub struct Transition {
    #[yaserde(prefix = "dto", rename = "action")]
    pub action: Option<String>,

    #[yaserde(prefix = "dto", rename = "actionId")]
    pub action_id: Option<i64>,

    #[yaserde(prefix = "dto", rename = "actionSequence")]
    pub action_sequence: Option<i64>,

    #[yaserde(prefix = "dto", rename = "label")]
    pub label: Option<String>,

    #[yaserde(prefix = "dto", rename = "nextState")]
    pub next_state: Option<String>,

    #[yaserde(prefix = "dto", rename = "subActions")]
    pub sub_actions: Option<ArrayOfSubAction>,
}

impl Validate for Transition {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfQueryResponseHelper")]
pub struct ArrayOfQueryResponseHelper {
    #[yaserde(prefix = "dto", rename = "QueryResponseHelper")]
    pub query_response_helper: Vec<QueryResponseHelper>,
}

impl Validate for ArrayOfQueryResponseHelper {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfRecordSection")]
pub struct ArrayOfRecordSection {
    #[yaserde(prefix = "dto", rename = "RecordSection")]
    pub record_section: Vec<RecordSection>,
}

impl Validate for ArrayOfRecordSection {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "dto", namespace = "dto: http://dto.ws.tririga.com", rename = "ArrayOfProject")]
pub struct ArrayOfProject {
    #[yaserde(prefix = "dto", rename = "Project")]
    pub project: Vec<Project>,
}

impl Validate for ArrayOfProject {}

