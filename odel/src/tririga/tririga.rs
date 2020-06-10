use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};
use xsd_types::types as xs;
use super::transport;
use super::transport::Validate;
use super::dto;
use super::content;
use super::gui::{ArrayOfGUI, Gui};

#[allow(dead_code)]

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://lang.java", rename = "Exception")]
pub struct Exception {
    pub base: Throwable,
}

impl Validate for Exception {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://lang.java", rename = "Throwable")]
pub struct Throwable {}

impl Validate for Throwable {}

//use   http://dto.ws.tririga.com;
//use   http://content.dto.ws.tririga.com;
//use   http://gui.dto.ws.tririga.com;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "ArrayOfLong")]
pub struct ArrayOfLong {
    #[yaserde(prefix = "tns", rename = "long")]
    pub long: Vec<i64>,
}

impl Validate for ArrayOfLong {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "ArrayOfString")]
pub struct ArrayOfString {
    #[yaserde(prefix = "tns", rename = "string")]
    pub string: Vec<String>,
}

impl Validate for ArrayOfString {}


// pub type GetListByTypes = GetListByTypes;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getListByTypes")]
pub struct GetListByTypes {
    #[yaserde(prefix = "tns", rename = "objectTypeIds")]
    pub object_type_ids: ArrayOfLong,
}

impl Validate for GetListByTypes {}


// pub type GetListByTypesResponse = GetListByTypesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getListByTypesResponse")]
pub struct GetListByTypesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfListType,
}

impl Validate for GetListByTypesResponse {}


// pub type GetRecordState = GetRecordState;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRecordState")]
pub struct GetRecordState {
    #[yaserde(prefix = "tns", rename = "recordIds")]
    pub record_ids: ArrayOfLong,
}

impl Validate for GetRecordState {}


// pub type GetRecordStateResponse = GetRecordStateResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRecordStateResponse")]
pub struct GetRecordStateResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfState,
}

impl Validate for GetRecordStateResponse {}


// pub type GetNamedQueryType = GetNamedQueryType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getNamedQueryType")]
pub struct GetNamedQueryType {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "queryName")]
    pub query_name: Option<String>,
}

impl Validate for GetNamedQueryType {}


// pub type GetNamedQueryTypeResponse = GetNamedQueryTypeResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getNamedQueryTypeResponse")]
pub struct GetNamedQueryTypeResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<String>,
}

impl Validate for GetNamedQueryTypeResponse {}


// pub type GetChecksum = GetChecksum;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getChecksum")]
pub struct GetChecksum {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: content::Content,
}

impl Validate for GetChecksum {}


// pub type GetChecksumResponse = GetChecksumResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getChecksumResponse")]
pub struct GetChecksumResponse {
    #[yaserde(prefix = "tns", rename = "checksum")]
    pub checksum: Option<i64>,
}

impl Validate for GetChecksumResponse {}


// pub type GetRequiredGuiFieldNames = GetRequiredGuiFieldNames;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRequiredGuiFieldNames")]
pub struct GetRequiredGuiFieldNames {
    #[yaserde(prefix = "tns", rename = "guiId")]
    pub gui_id: i64,
}

impl Validate for GetRequiredGuiFieldNames {}


// pub type GetRequiredGuiFieldNamesResponse = GetRequiredGuiFieldNamesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRequiredGuiFieldNamesResponse")]
pub struct GetRequiredGuiFieldNamesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for GetRequiredGuiFieldNamesResponse {}


// pub type GetRecordDataHeaders = GetRecordDataHeaders;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRecordDataHeaders")]
pub struct GetRecordDataHeaders {
    #[yaserde(prefix = "tns", rename = "recordIds")]
    pub record_ids: ArrayOfLong,
}

impl Validate for GetRecordDataHeaders {}


// pub type GetRecordDataHeadersResponse = GetRecordDataHeadersResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRecordDataHeadersResponse")]
pub struct GetRecordDataHeadersResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfRecord,
}

impl Validate for GetRecordDataHeadersResponse {}


// pub type GetCurrencyConversionRates = GetCurrencyConversionRates;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrencyConversionRates")]
pub struct GetCurrencyConversionRates {}

impl Validate for GetCurrencyConversionRates {}


// pub type GetCurrencyConversionRatesResponse = GetCurrencyConversionRatesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrencyConversionRatesResponse")]
pub struct GetCurrencyConversionRatesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfCurrencyConversionRate,
}

impl Validate for GetCurrencyConversionRatesResponse {}


// pub type GetObjectTypeListByModuleName = GetObjectTypeListByModuleName;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeListByModuleName")]
pub struct GetObjectTypeListByModuleName {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "isStandAlone")]
    pub is_stand_alone: bool,
}

impl Validate for GetObjectTypeListByModuleName {}


// pub type GetObjectTypeListByModuleNameResponse = GetObjectTypeListByModuleNameResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeListByModuleNameResponse")]
pub struct GetObjectTypeListByModuleNameResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfBaseObjectType,
}

impl Validate for GetObjectTypeListByModuleNameResponse {}


// pub type Register = Register;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "register")]
pub struct Register {
    #[yaserde(prefix = "tns", rename = "arg0")]
    pub arg_0: i64,
}

impl Validate for Register {}


// pub type RegisterResponse = RegisterResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "registerResponse")]
pub struct RegisterResponse {}

impl Validate for RegisterResponse {}


// pub type GetCurrencies = GetCurrencies;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrencies")]
pub struct GetCurrencies {}

impl Validate for GetCurrencies {}


// pub type GetCurrenciesResponse = GetCurrenciesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrenciesResponse")]
pub struct GetCurrenciesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfCurrency,
}

impl Validate for GetCurrenciesResponse {}


// pub type GetGUIsByName = GetGUIsByName;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIsByName")]
pub struct GetGUIsByName {
    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,
}

impl Validate for GetGUIsByName {}


// pub type GetGUIsByNameResponse = GetGUIsByNameResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIsByNameResponse")]
pub struct GetGUIsByNameResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: ArrayOfGUI,
}

impl Validate for GetGUIsByNameResponse {}


// pub type Upload = Upload;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "upload")]
pub struct Upload {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: content::Content,
}

impl Validate for Upload {}


// pub type UploadResponse = UploadResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "uploadResponse")]
pub struct UploadResponse {
    #[yaserde(prefix = "tns", rename = "return")]
    pub _return: Option<content::Response>,
}

impl Validate for UploadResponse {}


// pub type RunDynamicQuery = RunDynamicQuery;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runDynamicQuery")]
pub struct RunDynamicQuery {
    #[yaserde(prefix = "tns", rename = "projectName")]
    pub project_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeNames")]
    pub object_type_names: ArrayOfString,

    #[yaserde(prefix = "tns", rename = "guiNames")]
    pub gui_names: ArrayOfString,

    #[yaserde(prefix = "tns", rename = "associatedModuleName")]
    pub associated_module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "associatedObjectTypeName")]
    pub associated_object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "projectScope")]
    pub project_scope: i32,

    #[yaserde(prefix = "tns", rename = "displayFields")]
    pub display_fields: dto::ArrayOfDisplayLabel,

    #[yaserde(prefix = "tns", rename = "associatedDisplayFields")]
    pub associated_display_fields: dto::ArrayOfDisplayLabel,

    #[yaserde(prefix = "tns", rename = "fieldSortOrders")]
    pub field_sort_orders: dto::ArrayOfFieldSortOrder,

    #[yaserde(prefix = "tns", rename = "filters")]
    pub filters: dto::ArrayOfFilter,

    #[yaserde(prefix = "tns", rename = "associationFilters")]
    pub association_filters: dto::ArrayOfAssociationFilter,

    #[yaserde(prefix = "tns", rename = "start")]
    pub start: i32,

    #[yaserde(prefix = "tns", rename = "maximumResultCount")]
    pub maximum_result_count: i32,
}

impl Validate for RunDynamicQuery {}


// pub type RunDynamicQueryResponse = RunDynamicQueryResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runDynamicQueryResponse")]
pub struct RunDynamicQueryResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryResult>,
}

impl Validate for RunDynamicQueryResponse {}


// pub type GetHierarchyMetadata = GetHierarchyMetadata;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadata")]
pub struct GetHierarchyMetadata {
    #[yaserde(prefix = "tns", rename = "moduleId")]
    pub module_id: i64,

    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetHierarchyMetadata {}


// pub type GetHierarchyMetadataResponse = GetHierarchyMetadataResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadataResponse")]
pub struct GetHierarchyMetadataResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::HierarchyObjectType>,
}

impl Validate for GetHierarchyMetadataResponse {}


// pub type GetDefaultGUIActions = GetDefaultGUIActions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUIActions")]
pub struct GetDefaultGUIActions {
    #[yaserde(prefix = "tns", rename = "guiId")]
    pub gui_id: i64,
}

impl Validate for GetDefaultGUIActions {}


// pub type GetDefaultGUIActionsResponse = GetDefaultGUIActionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUIActionsResponse")]
pub struct GetDefaultGUIActionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfTransition,
}

impl Validate for GetDefaultGUIActionsResponse {}


// pub type GetListItems = GetListItems;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getListItems")]
pub struct GetListItems {
    #[yaserde(prefix = "tns", rename = "listId")]
    pub list_id: i64,
}

impl Validate for GetListItems {}


// pub type GetListItemsResponse = GetListItemsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getListItemsResponse")]
pub struct GetListItemsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfListItem,
}

impl Validate for GetListItemsResponse {}


// pub type GetGUIStateTransitions = GetGUIStateTransitions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIStateTransitions")]
pub struct GetGUIStateTransitions {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,

    #[yaserde(prefix = "tns", rename = "guiId")]
    pub gui_id: i64,
}

impl Validate for GetGUIStateTransitions {}


// pub type GetGUIStateTransitionsResponse = GetGUIStateTransitionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIStateTransitionsResponse")]
pub struct GetGUIStateTransitionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfState,
}

impl Validate for GetGUIStateTransitionsResponse {}


// pub type GetAssociationDefinitionsByName = GetAssociationDefinitionsByName;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociationDefinitionsByName")]
pub struct GetAssociationDefinitionsByName {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,
}

impl Validate for GetAssociationDefinitionsByName {}


// pub type GetAssociationDefinitionsByNameResponse = GetAssociationDefinitionsByNameResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociationDefinitionsByNameResponse")]
pub struct GetAssociationDefinitionsByNameResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAssociationDefinition,
}

impl Validate for GetAssociationDefinitionsByNameResponse {}


// pub type SaveRecord = SaveRecord;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "saveRecord")]
pub struct SaveRecord {
    #[yaserde(prefix = "tns", rename = "IntegrationRecords")]
    pub integration_records: dto::ArrayOfIntegrationRecord,
}

impl Validate for SaveRecord {}


// pub type SaveRecordResponse = SaveRecordResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "saveRecordResponse")]
pub struct SaveRecordResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for SaveRecordResponse {}


// pub type RunNamedQueryMultiBoContinue = RunNamedQueryMultiBoContinue;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBoContinue")]
pub struct RunNamedQueryMultiBoContinue {
    #[yaserde(prefix = "tns", rename = "continuationToken")]
    pub continuation_token: Option<dto::ContinuationToken>,
}

impl Validate for RunNamedQueryMultiBoContinue {}


// pub type RunNamedQueryMultiBoContinueResponse = RunNamedQueryMultiBoContinueResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBoContinueResponse")]
pub struct RunNamedQueryMultiBoContinueResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryMultiBoResult>,
}

impl Validate for RunNamedQueryMultiBoContinueResponse {}


// pub type GetDefaultGUI = GetDefaultGUI;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUI")]
pub struct GetDefaultGUI {
    #[yaserde(prefix = "tns", rename = "recordId")]
    pub record_id: i64,
}

impl Validate for GetDefaultGUI {}


// pub type GetDefaultGUIResponse = GetDefaultGUIResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUIResponse")]
pub struct GetDefaultGUIResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<Gui>,
}

impl Validate for GetDefaultGUIResponse {}


// pub type Copy = Copy;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "copy")]
pub struct Copy {
    #[yaserde(prefix = "tns", rename = "fromRecordId")]
    pub from_record_id: i64,

    #[yaserde(prefix = "tns", rename = "toRecordId")]
    pub to_record_id: i64,
}

impl Validate for Copy {}


// pub type CopyResponse = CopyResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "copyResponse")]
pub struct CopyResponse {
    #[yaserde(prefix = "tns", rename = "status")]
    pub status: Option<String>,
}

impl Validate for CopyResponse {}


// pub type GetObjectTypeId = GetObjectTypeId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeId")]
pub struct GetObjectTypeId {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,
}

impl Validate for GetObjectTypeId {}


// pub type GetObjectTypeIdResponse = GetObjectTypeIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeIdResponse")]
pub struct GetObjectTypeIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: i64,
}

impl Validate for GetObjectTypeIdResponse {}


// pub type DeleteCurrencyConversionRates = DeleteCurrencyConversionRates;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "deleteCurrencyConversionRates")]
pub struct DeleteCurrencyConversionRates {
    #[yaserde(prefix = "tns", rename = "conversionRateIds")]
    pub conversion_rate_ids: ArrayOfLong,
}

impl Validate for DeleteCurrencyConversionRates {}


// pub type DeleteCurrencyConversionRatesResponse = DeleteCurrencyConversionRatesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "deleteCurrencyConversionRatesResponse")]
pub struct DeleteCurrencyConversionRatesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for DeleteCurrencyConversionRatesResponse {}


// pub type GetHierarchyMetadataByModuleId = GetHierarchyMetadataByModuleId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadataByModuleId")]
pub struct GetHierarchyMetadataByModuleId {
    #[yaserde(prefix = "tns", rename = "moduleId")]
    pub module_id: i64,
}

impl Validate for GetHierarchyMetadataByModuleId {}


// pub type GetHierarchyMetadataByModuleIdResponse = GetHierarchyMetadataByModuleIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadataByModuleIdResponse")]
pub struct GetHierarchyMetadataByModuleIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::HierarchyObjectType>,
}

impl Validate for GetHierarchyMetadataByModuleIdResponse {}


// pub type GetProjectId = GetProjectId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getProjectId")]
pub struct GetProjectId {
    #[yaserde(prefix = "tns", rename = "projectName")]
    pub project_name: Option<String>,
}

impl Validate for GetProjectId {}


// pub type GetProjectIdResponse = GetProjectIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getProjectIdResponse")]
pub struct GetProjectIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: i64,
}

impl Validate for GetProjectIdResponse {}


// pub type RunNamedQueryMultiBoLocalized = RunNamedQueryMultiBoLocalized;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBoLocalized")]
pub struct RunNamedQueryMultiBoLocalized {
    #[yaserde(prefix = "tns", rename = "projectName")]
    pub project_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "queryName")]
    pub query_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "filters")]
    pub filters: dto::ArrayOfFilter,

    #[yaserde(prefix = "tns", rename = "start")]
    pub start: i32,

    #[yaserde(prefix = "tns", rename = "maximumResultCount")]
    pub maximum_result_count: i32,
}

impl Validate for RunNamedQueryMultiBoLocalized {}


// pub type RunNamedQueryMultiBoLocalizedResponse = RunNamedQueryMultiBoLocalizedResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBoLocalizedResponse")]
pub struct RunNamedQueryMultiBoLocalizedResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryMultiBoResult>,
}

impl Validate for RunNamedQueryMultiBoLocalizedResponse {}


// pub type GetAssociatedRecords = GetAssociatedRecords;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociatedRecords")]
pub struct GetAssociatedRecords {
    #[yaserde(prefix = "tns", rename = "recordId")]
    pub record_id: i64,

    #[yaserde(prefix = "tns", rename = "associationString")]
    pub association_string: Option<String>,

    #[yaserde(prefix = "tns", rename = "maxResults")]
    pub max_results: i32,
}

impl Validate for GetAssociatedRecords {}


// pub type GetAssociatedRecordsResponse = GetAssociatedRecordsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociatedRecordsResponse")]
pub struct GetAssociatedRecordsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAssociation,
}

impl Validate for GetAssociatedRecordsResponse {}


// pub type GetAssociationDefinitions = GetAssociationDefinitions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociationDefinitions")]
pub struct GetAssociationDefinitions {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetAssociationDefinitions {}


// pub type GetAssociationDefinitionsResponse = GetAssociationDefinitionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAssociationDefinitionsResponse")]
pub struct GetAssociationDefinitionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAssociationDefinition,
}

impl Validate for GetAssociationDefinitionsResponse {}


// pub type Delete = Delete;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "delete")]
pub struct Delete {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: content::Content,
}

impl Validate for Delete {}


// pub type DeleteResponse = DeleteResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "deleteResponse")]
pub struct DeleteResponse {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: Option<content::Response>,
}

impl Validate for DeleteResponse {}


// pub type RunNamedQuery = RunNamedQuery;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQuery")]
pub struct RunNamedQuery {
    #[yaserde(prefix = "tns", rename = "projectName")]
    pub project_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "queryName")]
    pub query_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "filters")]
    pub filters: dto::ArrayOfFilter,

    #[yaserde(prefix = "tns", rename = "start")]
    pub start: i32,

    #[yaserde(prefix = "tns", rename = "maximumResultCount")]
    pub maximum_result_count: i32,
}

impl Validate for RunNamedQuery {}


// pub type RunNamedQueryResponse = RunNamedQueryResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryResponse")]
pub struct RunNamedQueryResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryResult>,
}

impl Validate for RunNamedQueryResponse {}


// pub type RunNamedQueryMultiBo = RunNamedQueryMultiBo;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBo")]
pub struct RunNamedQueryMultiBo {
    #[yaserde(prefix = "tns", rename = "projectName")]
    pub project_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "queryName")]
    pub query_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "filters")]
    pub filters: dto::ArrayOfFilter,

    #[yaserde(prefix = "tns", rename = "start")]
    pub start: i32,

    #[yaserde(prefix = "tns", rename = "maximumResultCount")]
    pub maximum_result_count: i32,
}

impl Validate for RunNamedQueryMultiBo {}


// pub type RunNamedQueryMultiBoResponse = RunNamedQueryMultiBoResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryMultiBoResponse")]
pub struct RunNamedQueryMultiBoResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryMultiBoResult>,
}

impl Validate for RunNamedQueryMultiBoResponse {}


// pub type GetModules = GetModules;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getModules")]
pub struct GetModules {}

impl Validate for GetModules {}


// pub type GetModulesResponse = GetModulesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getModulesResponse")]
pub struct GetModulesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfModule,
}

impl Validate for GetModulesResponse {}


// pub type GetContentFieldsDefinition = GetContentFieldsDefinition;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getContentFieldsDefinition")]
pub struct GetContentFieldsDefinition {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: String,

    #[yaserde(prefix = "tns", rename = "boName")]
    pub bo_name: String,
}

impl Validate for GetContentFieldsDefinition {}


// pub type GetContentFieldsDefinitionResponse = GetContentFieldsDefinitionResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getContentFieldsDefinitionResponse")]
pub struct GetContentFieldsDefinitionResponse {
    #[yaserde(prefix = "tns", rename = "contentFields")]
    pub content_fields: content::ArrayOfContentField,
}

impl Validate for GetContentFieldsDefinitionResponse {}


// pub type KeywordSearch = KeywordSearch;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "keywordSearch")]
pub struct KeywordSearch {
    #[yaserde(prefix = "tns", rename = "projectId")]
    pub project_id: i64,

    #[yaserde(prefix = "tns", rename = "moduleId")]
    pub module_id: i64,

    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,

    #[yaserde(prefix = "tns", rename = "keyword")]
    pub keyword: Option<String>,
}

impl Validate for KeywordSearch {}


// pub type KeywordSearchResponse = KeywordSearchResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "keywordSearchResponse")]
pub struct KeywordSearchResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfKeywordSearchResult,
}

impl Validate for KeywordSearchResponse {}


// pub type GetGUI = GetGUI;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUI")]
pub struct GetGUI {
    #[yaserde(prefix = "tns", rename = "guiId")]
    pub gui_id: i64,

    #[yaserde(prefix = "tns", rename = "recordId")]
    pub record_id: i64,
}

impl Validate for GetGUI {}


// pub type GetGUIResponse = GetGUIResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIResponse")]
pub struct GetGUIResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<Gui>,
}

impl Validate for GetGUIResponse {}


// pub type GetModuleId = GetModuleId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getModuleId")]
pub struct GetModuleId {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,
}

impl Validate for GetModuleId {}


// pub type GetModuleIdResponse = GetModuleIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getModuleIdResponse")]
pub struct GetModuleIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: i32,
}

impl Validate for GetModuleIdResponse {}


// pub type PutCurrencyConversionRates = PutCurrencyConversionRates;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "putCurrencyConversionRates")]
pub struct PutCurrencyConversionRates {
    #[yaserde(prefix = "tns", rename = "CurrencyConversionRates")]
    pub currency_conversion_rates: dto::ArrayOfCurrencyConversionRate,
}

impl Validate for PutCurrencyConversionRates {}


// pub type PutCurrencyConversionRatesResponse = PutCurrencyConversionRatesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "putCurrencyConversionRatesResponse")]
pub struct PutCurrencyConversionRatesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfCurrencyConversionRate,
}

impl Validate for PutCurrencyConversionRatesResponse {}


// pub type Download = Download;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "download")]
pub struct Download {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: content::Content,
}

impl Validate for Download {}


// pub type DownloadResponse = DownloadResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "downloadResponse")]
pub struct DownloadResponse {
    #[yaserde(prefix = "tns", rename = "return")]
    pub _return: Option<content::Response>,
}

impl Validate for DownloadResponse {}


// pub type TriggerActions = TriggerActions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "triggerActions")]
pub struct TriggerActions {
    #[yaserde(prefix = "tns", rename = "TriggerActions")]
    pub trigger_actions: dto::ArrayOfTriggerActions,
}

impl Validate for TriggerActions {}


// pub type TriggerActionsResponse = TriggerActionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "triggerActionsResponse")]
pub struct TriggerActionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for TriggerActionsResponse {}


// pub type DeassociateRecords = DeassociateRecords;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "deassociateRecords")]
pub struct DeassociateRecords {
    #[yaserde(prefix = "tns", rename = "Associations")]
    pub associations: dto::ArrayOfAssociation,
}

impl Validate for DeassociateRecords {}


// pub type DeassociateRecordsResponse = DeassociateRecordsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "deassociateRecordsResponse")]
pub struct DeassociateRecordsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for DeassociateRecordsResponse {}


// pub type AcceptActionItems = AcceptActionItems;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "acceptActionItems")]
pub struct AcceptActionItems {
    #[yaserde(prefix = "tns", rename = "ActionItems")]
    pub action_items: dto::ArrayOfActionItem,
}

impl Validate for AcceptActionItems {}


// pub type AcceptActionItemsResponse = AcceptActionItemsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "acceptActionItemsResponse")]
pub struct AcceptActionItemsResponse {}

impl Validate for AcceptActionItemsResponse {}


// pub type RunNamedMetricQueryContinue = RunNamedMetricQueryContinue;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedMetricQueryContinue")]
pub struct RunNamedMetricQueryContinue {
    #[yaserde(prefix = "tns", rename = "continuationToken")]
    pub continuation_token: Option<dto::ContinuationToken>,
}

impl Validate for RunNamedMetricQueryContinue {}


// pub type RunNamedMetricQueryContinueResponse = RunNamedMetricQueryContinueResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedMetricQueryContinueResponse")]
pub struct RunNamedMetricQueryContinueResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::MetricQueryResult>,
}

impl Validate for RunNamedMetricQueryContinueResponse {}


// pub type UploadFrom = UploadFrom;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "uploadFrom")]
pub struct UploadFrom {
    #[yaserde(prefix = "tns", rename = "contents")]
    pub contents: content::ArrayOfContent1,
}

impl Validate for UploadFrom {}


// pub type UploadFromResponse = UploadFromResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "uploadFromResponse")]
pub struct UploadFromResponse {
    #[yaserde(prefix = "tns", rename = "reponses")]
    pub reponses: content::ArrayOfResponse,
}

impl Validate for UploadFromResponse {}


// pub type GetObjectTypeListByModuleId = GetObjectTypeListByModuleId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeListByModuleId")]
pub struct GetObjectTypeListByModuleId {
    #[yaserde(prefix = "tns", rename = "moduleId")]
    pub module_id: i64,

    #[yaserde(prefix = "tns", rename = "isStandAlone")]
    pub is_stand_alone: bool,
}

impl Validate for GetObjectTypeListByModuleId {}


// pub type GetObjectTypeListByModuleIdResponse = GetObjectTypeListByModuleIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeListByModuleIdResponse")]
pub struct GetObjectTypeListByModuleIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfBaseObjectType,
}

impl Validate for GetObjectTypeListByModuleIdResponse {}


// pub type RunDynamicQueryContinue = RunDynamicQueryContinue;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runDynamicQueryContinue")]
pub struct RunDynamicQueryContinue {
    #[yaserde(prefix = "tns", rename = "continuationToken")]
    pub continuation_token: Option<dto::ContinuationToken>,
}

impl Validate for RunDynamicQueryContinue {}


// pub type RunDynamicQueryContinueResponse = RunDynamicQueryContinueResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runDynamicQueryContinueResponse")]
pub struct RunDynamicQueryContinueResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryResult>,
}

impl Validate for RunDynamicQueryContinueResponse {}


// pub type GetDefaultGuiId = GetDefaultGuiId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGuiId")]
pub struct GetDefaultGuiId {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetDefaultGuiId {}


// pub type GetDefaultGuiIdResponse = GetDefaultGuiIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGuiIdResponse")]
pub struct GetDefaultGuiIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: i64,
}

impl Validate for GetDefaultGuiIdResponse {}


// pub type GetObjectTypeActions = GetObjectTypeActions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeActions")]
pub struct GetObjectTypeActions {
    #[yaserde(prefix = "tns", rename = "moduleId")]
    pub module_id: i64,

    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,

    #[yaserde(prefix = "tns", rename = "recordId")]
    pub record_id: i64,

    #[yaserde(prefix = "tns", rename = "guiId")]
    pub gui_id: i64,
}

impl Validate for GetObjectTypeActions {}


// pub type GetObjectTypeActionsResponse = GetObjectTypeActionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeActionsResponse")]
pub struct GetObjectTypeActionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ObjectTypeAction>,
}

impl Validate for GetObjectTypeActionsResponse {}


// pub type GetCurrencyConversionGroups = GetCurrencyConversionGroups;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrencyConversionGroups")]
pub struct GetCurrencyConversionGroups {}

impl Validate for GetCurrencyConversionGroups {}


// pub type GetCurrencyConversionGroupsResponse = GetCurrencyConversionGroupsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getCurrencyConversionGroupsResponse")]
pub struct GetCurrencyConversionGroupsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfConversionGroup,
}

impl Validate for GetCurrencyConversionGroupsResponse {}


// pub type GetObjectType = GetObjectType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectType")]
pub struct GetObjectType {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetObjectType {}


// pub type GetObjectTypeResponse = GetObjectTypeResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeResponse")]
pub struct GetObjectTypeResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ObjectType>,
}

impl Validate for GetObjectTypeResponse {}


// pub type GetProjects = GetProjects;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getProjects")]
pub struct GetProjects {}

impl Validate for GetProjects {}


// pub type GetProjectsResponse = GetProjectsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getProjectsResponse")]
pub struct GetProjectsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfProject,
}

impl Validate for GetProjectsResponse {}


// pub type GetRootRecordId = GetRootRecordId;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRootRecordId")]
pub struct GetRootRecordId {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,
}

impl Validate for GetRootRecordId {}


// pub type GetRootRecordIdResponse = GetRootRecordIdResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getRootRecordIdResponse")]
pub struct GetRootRecordIdResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelper>,
}

impl Validate for GetRootRecordIdResponse {}


// pub type GetActionItems = GetActionItems;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getActionItems")]
pub struct GetActionItems {}

impl Validate for GetActionItems {}


// pub type GetActionItemsResponse = GetActionItemsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getActionItemsResponse")]
pub struct GetActionItemsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfActionItem,
}

impl Validate for GetActionItemsResponse {}


// pub type GetHttpSession = GetHttpSession;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHttpSession")]
pub struct GetHttpSession {}

impl Validate for GetHttpSession {}


// pub type GetHttpSessionResponse = GetHttpSessionResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHttpSessionResponse")]
pub struct GetHttpSessionResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::HttpSession>,
}

impl Validate for GetHttpSessionResponse {}


// pub type GetAllAssociatedRecords = GetAllAssociatedRecords;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAllAssociatedRecords")]
pub struct GetAllAssociatedRecords {
    #[yaserde(prefix = "tns", rename = "recordIds")]
    pub record_ids: ArrayOfLong,

    #[yaserde(prefix = "tns", rename = "associationString")]
    pub association_string: Option<String>,

    #[yaserde(prefix = "tns", rename = "maxResults")]
    pub max_results: i32,
}

impl Validate for GetAllAssociatedRecords {}


// pub type GetAllAssociatedRecordsResponse = GetAllAssociatedRecordsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAllAssociatedRecordsResponse")]
pub struct GetAllAssociatedRecordsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAssociationRecord,
}

impl Validate for GetAllAssociatedRecordsResponse {}


// pub type AssociateRecords = AssociateRecords;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "associateRecords")]
pub struct AssociateRecords {
    #[yaserde(prefix = "tns", rename = "Associations")]
    pub associations: dto::ArrayOfAssociation,
}

impl Validate for AssociateRecords {}


// pub type AssociateRecordsResponse = AssociateRecordsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "associateRecordsResponse")]
pub struct AssociateRecordsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelperHeader>,
}

impl Validate for AssociateRecordsResponse {}


// pub type GetContentLength = GetContentLength;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getContentLength")]
pub struct GetContentLength {
    #[yaserde(prefix = "tns", rename = "content")]
    pub content: content::Content,
}

impl Validate for GetContentLength {}


// pub type GetContentLengthResponse = GetContentLengthResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getContentLengthResponse")]
pub struct GetContentLengthResponse {
    #[yaserde(prefix = "tns", rename = "length")]
    pub length: Option<i64>,
}

impl Validate for GetContentLengthResponse {}


// pub type RunNamedMetricQuery = RunNamedMetricQuery;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedMetricQuery")]
pub struct RunNamedMetricQuery {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "queryName")]
    pub query_name: Option<String>,

    #[yaserde(prefix = "tns", rename = "metricQueryfilters")]
    pub metric_queryfilters: dto::ArrayOfMetricQueryFilter,

    #[yaserde(prefix = "tns", rename = "groupBy")]
    pub group_by: Option<dto::GroupBy>,

    #[yaserde(prefix = "tns", rename = "start")]
    pub start: i32,

    #[yaserde(prefix = "tns", rename = "maximumResultCount")]
    pub maximum_result_count: i32,
}

impl Validate for RunNamedMetricQuery {}


// pub type RunNamedMetricQueryResponse = RunNamedMetricQueryResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedMetricQueryResponse")]
pub struct RunNamedMetricQueryResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::MetricQueryResult>,
}

impl Validate for RunNamedMetricQueryResponse {}


// pub type DownloadTo = DownloadTo;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "downloadTo")]
pub struct DownloadTo {
    #[yaserde(prefix = "tns", rename = "contents")]
    pub contents: content::ArrayOfContent1,
}

impl Validate for DownloadTo {}


// pub type DownloadToResponse = DownloadToResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "downloadToResponse")]
pub struct DownloadToResponse {
    #[yaserde(prefix = "tns", rename = "reponses")]
    pub reponses: content::ArrayOfResponse,
}

impl Validate for DownloadToResponse {}


// pub type GetHierarchyMetadataByModuleName = GetHierarchyMetadataByModuleName;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadataByModuleName")]
pub struct GetHierarchyMetadataByModuleName {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: Option<String>,
}

impl Validate for GetHierarchyMetadataByModuleName {}


// pub type GetHierarchyMetadataByModuleNameResponse = GetHierarchyMetadataByModuleNameResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getHierarchyMetadataByModuleNameResponse")]
pub struct GetHierarchyMetadataByModuleNameResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::HierarchyObjectType>,
}

impl Validate for GetHierarchyMetadataByModuleNameResponse {}


// pub type GetChildren = GetChildren;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getChildren")]
pub struct GetChildren {
    #[yaserde(prefix = "tns", rename = "recordId")]
    pub record_id: i64,
}

impl Validate for GetChildren {}


// pub type GetChildrenResponse = GetChildrenResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getChildrenResponse")]
pub struct GetChildrenResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfChildRecord,
}

impl Validate for GetChildrenResponse {}


// pub type GetTargetRecords = GetTargetRecords;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getTargetRecords")]
pub struct GetTargetRecords {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,

    #[yaserde(prefix = "tns", rename = "sectionName")]
    pub section_name: Option<String>,
}

impl Validate for GetTargetRecords {}


// pub type GetTargetRecordsResponse = GetTargetRecordsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getTargetRecordsResponse")]
pub struct GetTargetRecordsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAssociation,
}

impl Validate for GetTargetRecordsResponse {}


// pub type TerminateSession = TerminateSession;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "terminateSession")]
pub struct TerminateSession {
    #[yaserde(prefix = "tns", rename = "id")]
    pub id: i64,
}

impl Validate for TerminateSession {}


// pub type TerminateSessionResponse = TerminateSessionResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "terminateSessionResponse")]
pub struct TerminateSessionResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ResponseHelper>,
}

impl Validate for TerminateSessionResponse {}


// pub type GetApplicationInfo = GetApplicationInfo;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getApplicationInfo")]
pub struct GetApplicationInfo {}

impl Validate for GetApplicationInfo {}


// pub type GetApplicationInfoResponse = GetApplicationInfoResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getApplicationInfoResponse")]
pub struct GetApplicationInfoResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ApplicationInfo>,
}

impl Validate for GetApplicationInfoResponse {}


// pub type GetDefaultGUIStructure = GetDefaultGUIStructure;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUIStructure")]
pub struct GetDefaultGUIStructure {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetDefaultGUIStructure {}


// pub type GetDefaultGUIStructureResponse = GetDefaultGUIStructureResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getDefaultGUIStructureResponse")]
pub struct GetDefaultGUIStructureResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<Gui>,
}

impl Validate for GetDefaultGUIStructureResponse {}


// pub type GetObjectTypeByName = GetObjectTypeByName;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeByName")]
pub struct GetObjectTypeByName {
    #[yaserde(prefix = "tns", rename = "moduleName")]
    pub module_name: String,

    #[yaserde(prefix = "tns", rename = "objectTypeName")]
    pub object_type_name: String,
}

impl Validate for GetObjectTypeByName {}


// pub type GetObjectTypeByNameResponse = GetObjectTypeByNameResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getObjectTypeByNameResponse")]
pub struct GetObjectTypeByNameResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::ObjectType>,
}

impl Validate for GetObjectTypeByNameResponse {}


// pub type GetAvailableActions = GetAvailableActions;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAvailableActions")]
pub struct GetAvailableActions {
    #[yaserde(prefix = "tns", rename = "recordIds")]
    pub record_ids: ArrayOfLong,
}

impl Validate for GetAvailableActions {}


// pub type GetAvailableActionsResponse = GetAvailableActionsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getAvailableActionsResponse")]
pub struct GetAvailableActionsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfAvailableAction,
}

impl Validate for GetAvailableActionsResponse {}


// pub type GetUserLicenses = GetUserLicenses;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getUserLicenses")]
pub struct GetUserLicenses {}

impl Validate for GetUserLicenses {}


// pub type GetUserLicensesResponse = GetUserLicensesResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getUserLicensesResponse")]
pub struct GetUserLicensesResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: dto::ArrayOfResponseHelper,
}

impl Validate for GetUserLicensesResponse {}


// pub type RunNamedQueryContinue = RunNamedQueryContinue;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryContinue")]
pub struct RunNamedQueryContinue {
    #[yaserde(prefix = "tns", rename = "continuationToken")]
    pub continuation_token: Option<dto::ContinuationToken>,
}

impl Validate for RunNamedQueryContinue {}


// pub type RunNamedQueryContinueResponse = RunNamedQueryContinueResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "runNamedQueryContinueResponse")]
pub struct RunNamedQueryContinueResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: Option<dto::QueryResult>,
}

impl Validate for RunNamedQueryContinueResponse {}


// pub type GetGUIs = GetGUIs;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIs")]
pub struct GetGUIs {
    #[yaserde(prefix = "tns", rename = "objectTypeId")]
    pub object_type_id: i64,
}

impl Validate for GetGUIs {}


// pub type GetGUIsResponse = GetGUIsResponse;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ws.tririga.com", rename = "getGUIsResponse")]
pub struct GetGUIsResponse {
    #[yaserde(prefix = "tns", rename = "out")]
    pub out: ArrayOfGUI,
}

impl Validate for GetGUIsResponse {}


// pub type InvalidArgumentException = ns3::InvalidArgumentException;
// pub type ObjectTypeDoesNotExistException = ns3::ObjectTypeDoesNotExistException;
// pub type Exception = ns4::Exception;
// pub type ModuleDoesNotExistException = ns3::ModuleDoesNotExistException;
// pub type QueryDoesNotExistException = ns3::QueryDoesNotExistException;
// pub type InvalidDocumentTypeException = content::InvalidDocumentTypeException;
// pub type InvalidContentException = content::InvalidContentException;
// pub type AccessException = ns3::AccessException;
// pub type RecordDoesNotExistException = ns3::RecordDoesNotExistException;
// pub type ProjectDoesNotExistException = ns3::ProjectDoesNotExistException;
// pub type GuiDoesNotExistException = ns3::GuiDoesNotExistException;
// pub type ListDoesNotExistException = ns3::ListDoesNotExistException;
// pub type InvalidContinuationTokenException = ns3::InvalidContinuationTokenException;
// pub type ActionDoesNotExistException = ns3::ActionDoesNotExistException;
// pub type ActionItemInvalidException = ns3::ActionItemInvalidException;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ProjectDoesNotExistException")]
pub struct ProjectDoesNotExistException {}

impl Validate for ProjectDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "InvalidContinuationTokenException")]
pub struct InvalidContinuationTokenException {}

impl Validate for InvalidContinuationTokenException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "QueryDoesNotExistException")]
pub struct QueryDoesNotExistException {}

impl Validate for QueryDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ObjectTypeDoesNotExistException")]
pub struct ObjectTypeDoesNotExistException {}

impl Validate for ObjectTypeDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "InvalidArgumentException")]
pub struct InvalidArgumentException {}

impl Validate for InvalidArgumentException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "RecordDoesNotExistException")]
pub struct RecordDoesNotExistException {}

impl Validate for RecordDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "GuiDoesNotExistException")]
pub struct GuiDoesNotExistException {}

impl Validate for GuiDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ActionDoesNotExistException")]
pub struct ActionDoesNotExistException {}

impl Validate for ActionDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ModuleDoesNotExistException")]
pub struct ModuleDoesNotExistException {}

impl Validate for ModuleDoesNotExistException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "AccessException")]
pub struct AccessException {}

impl Validate for AccessException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ActionItemInvalidException")]
pub struct ActionItemInvalidException {}

impl Validate for ActionItemInvalidException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://errors.ws.tririga.com", rename = "ListDoesNotExistException")]
pub struct ListDoesNotExistException {}

impl Validate for ListDoesNotExistException {}


pub async fn get_list_by_types<T: transport::Transport>(
    transport: &T,
    request: &GetListByTypes
) -> Result<GetListByTypesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_record_state<T: transport::Transport>(
    transport: &T,
    request: &GetRecordState
) -> Result<GetRecordStateResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_named_query_type<T: transport::Transport>(
    transport: &T,
    request: &GetNamedQueryType
) -> Result<GetNamedQueryTypeResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_checksum<T: transport::Transport>(
    transport: &T,
    request: &GetChecksum
) -> Result<GetChecksumResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_required_gui_field_names<T: transport::Transport>(
    transport: &T,
    request: &GetRequiredGuiFieldNames
) -> Result<GetRequiredGuiFieldNamesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_record_data_headers<T: transport::Transport>(
    transport: &T,
    request: &GetRecordDataHeaders
) -> Result<GetRecordDataHeadersResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_currency_conversion_rates<T: transport::Transport>(
    transport: &T,
    request: &GetCurrencyConversionRates
) -> Result<GetCurrencyConversionRatesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type_list_by_module_name<T: transport::Transport>(
    transport: &T,
    request: &GetObjectTypeListByModuleName
) -> Result<GetObjectTypeListByModuleNameResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn register<T: transport::Transport>(
    transport: &T,
    request: &Register
) -> Result<RegisterResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_currencies<T: transport::Transport>(
    transport: &T,
    request: &GetCurrencies
) -> Result<GetCurrenciesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_gu_is_by_name<T: transport::Transport>(
    transport: &T,
    request: &GetGUIsByName
) -> Result<GetGUIsByNameResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn upload<T: transport::Transport>(
    transport: &T,
    request: &Upload
) -> Result<UploadResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_dynamic_query<T: transport::Transport>(
    transport: &T,
    request: &RunDynamicQuery
) -> Result<RunDynamicQueryResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_hierarchy_metadata<T: transport::Transport>(
    transport: &T,
    request: &GetHierarchyMetadata
) -> Result<GetHierarchyMetadataResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_default_gui_actions<T: transport::Transport>(
    transport: &T,
    request: &GetDefaultGUIActions
) -> Result<GetDefaultGUIActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_list_items<T: transport::Transport>(
    transport: &T,
    request: &GetListItems
) -> Result<GetListItemsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_gui_state_transitions<T: transport::Transport>(
    transport: &T,
    request: &GetGUIStateTransitions
) -> Result<GetGUIStateTransitionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_association_definitions_by_name<T: transport::Transport>(
    transport: &T,
    request: &GetAssociationDefinitionsByName
) -> Result<GetAssociationDefinitionsByNameResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn save_record<T: transport::Transport>(
    transport: &T,
    request: &SaveRecord
) -> Result<SaveRecordResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_query_multi_bo_continue<T: transport::Transport>(
    transport: &T,
    request: &RunNamedQueryMultiBoContinue
) -> Result<RunNamedQueryMultiBoContinueResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_default_gui<T: transport::Transport>(
    transport: &T,
    request: &GetDefaultGUI
) -> Result<GetDefaultGUIResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn copy<T: transport::Transport>(
    transport: &T,
    request: &Copy
) -> Result<CopyResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type_id<T: transport::Transport>(
    transport: &T,
    request: &GetObjectTypeId
) -> Result<GetObjectTypeIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn delete_currency_conversion_rates<T: transport::Transport>(
    transport: &T,
    request: &DeleteCurrencyConversionRates
) -> Result<DeleteCurrencyConversionRatesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_hierarchy_metadata_by_module_id<T: transport::Transport>(
    transport: &T,
    request: &GetHierarchyMetadataByModuleId
) -> Result<GetHierarchyMetadataByModuleIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_project_id<T: transport::Transport>(
    transport: &T,
    request: &GetProjectId
) -> Result<GetProjectIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_query_multi_bo_localized<T: transport::Transport>(
    transport: &T,
    request: &RunNamedQueryMultiBoLocalized
) -> Result<RunNamedQueryMultiBoLocalizedResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_associated_records<T: transport::Transport>(
    transport: &T,
    request: &GetAssociatedRecords
) -> Result<GetAssociatedRecordsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_association_definitions<T: transport::Transport>(
    transport: &T,
    request: &GetAssociationDefinitions
) -> Result<GetAssociationDefinitionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn delete<T: transport::Transport>(
    transport: &T,
    request: &Delete
) -> Result<DeleteResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_query<T: transport::Transport>(
    transport: &T,
    request: &RunNamedQuery
) -> Result<RunNamedQueryResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_query_multi_bo<T: transport::Transport>(
    transport: &T,
    request: &RunNamedQueryMultiBo
) -> Result<RunNamedQueryMultiBoResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_modules<T: transport::Transport>(
    transport: &T,
    request: &GetModules
) -> Result<GetModulesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_content_fields_definition<T: transport::Transport>(
    transport: &T,
    request: &GetContentFieldsDefinition
) -> Result<GetContentFieldsDefinitionResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn keyword_search<T: transport::Transport>(
    transport: &T,
    request: &KeywordSearch
) -> Result<KeywordSearchResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_gui<T: transport::Transport>(
    transport: &T,
    request: &GetGUI
) -> Result<GetGUIResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_module_id<T: transport::Transport>(
    transport: &T,
    request: &GetModuleId
) -> Result<GetModuleIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn put_currency_conversion_rates<T: transport::Transport>(
    transport: &T,
    request: &PutCurrencyConversionRates
) -> Result<PutCurrencyConversionRatesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn download<T: transport::Transport>(
    transport: &T,
    request: &Download
) -> Result<DownloadResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn trigger_actions<T: transport::Transport>(
    transport: &T,
    request: &TriggerActions
) -> Result<TriggerActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn deassociate_records<T: transport::Transport>(
    transport: &T,
    request: &DeassociateRecords
) -> Result<DeassociateRecordsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn accept_action_items<T: transport::Transport>(
    transport: &T,
    request: &AcceptActionItems
) -> Result<AcceptActionItemsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_metric_query_continue<T: transport::Transport>(
    transport: &T,
    request: &RunNamedMetricQueryContinue
) -> Result<RunNamedMetricQueryContinueResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn upload_from<T: transport::Transport>(
    transport: &T,
    request: &UploadFrom
) -> Result<UploadFromResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type_list_by_module_id<T: transport::Transport>(
    transport: &T,
    request: &GetObjectTypeListByModuleId
) -> Result<GetObjectTypeListByModuleIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_dynamic_query_continue<T: transport::Transport>(
    transport: &T,
    request: &RunDynamicQueryContinue
) -> Result<RunDynamicQueryContinueResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_default_gui_id<T: transport::Transport>(
    transport: &T,
    request: &GetDefaultGuiId
) -> Result<GetDefaultGuiIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type_actions<T: transport::Transport>(
    transport: &T,
    request: &GetObjectTypeActions
) -> Result<GetObjectTypeActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_currency_conversion_groups<T: transport::Transport>(
    transport: &T,
    request: &GetCurrencyConversionGroups
) -> Result<GetCurrencyConversionGroupsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type<T: transport::Transport>(
    transport: &T,
    request: &GetObjectType
) -> Result<GetObjectTypeResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_projects<T: transport::Transport>(
    transport: &T,
    request: &GetProjects
) -> Result<GetProjectsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_root_record_id<T: transport::Transport>(
    transport: &T,
    request: &GetRootRecordId
) -> Result<GetRootRecordIdResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_action_items<T: transport::Transport>(
    transport: &T,
    request: &GetActionItems
) -> Result<GetActionItemsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_http_session<T: transport::Transport>(
    transport: &T,
    request: &GetHttpSession
) -> Result<GetHttpSessionResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_all_associated_records<T: transport::Transport>(
    transport: &T,
    request: &GetAllAssociatedRecords
) -> Result<GetAllAssociatedRecordsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn associate_records<T: transport::Transport>(
    transport: &T,
    request: &AssociateRecords
) -> Result<AssociateRecordsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_content_length<T: transport::Transport>(
    transport: &T,
    request: &GetContentLength
) -> Result<GetContentLengthResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_metric_query<T: transport::Transport>(
    transport: &T,
    request: &RunNamedMetricQuery
) -> Result<RunNamedMetricQueryResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn download_to<T: transport::Transport>(
    transport: &T,
    request: &DownloadTo
) -> Result<DownloadToResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_hierarchy_metadata_by_module_name<T: transport::Transport>(
    transport: &T,
    request: &GetHierarchyMetadataByModuleName
) -> Result<GetHierarchyMetadataByModuleNameResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_children<T: transport::Transport>(
    transport: &T,
    request: &GetChildren
) -> Result<GetChildrenResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_target_records<T: transport::Transport>(
    transport: &T,
    request: &GetTargetRecords
) -> Result<GetTargetRecordsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn terminate_session<T: transport::Transport>(
    transport: &T,
    request: &TerminateSession
) -> Result<TerminateSessionResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_application_info<T: transport::Transport>(
    transport: &T,
    request: &GetApplicationInfo
) -> Result<GetApplicationInfoResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_default_gui_structure<T: transport::Transport>(
    transport: &T,
    request: &GetDefaultGUIStructure
) -> Result<GetDefaultGUIStructureResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_object_type_by_name<T: transport::Transport>(
    transport: &T,
    request: &GetObjectTypeByName
) -> Result<GetObjectTypeByNameResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_available_actions<T: transport::Transport>(
    transport: &T,
    request: &GetAvailableActions
) -> Result<GetAvailableActionsResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_user_licenses<T: transport::Transport>(
    transport: &T,
    request: &GetUserLicenses
) -> Result<GetUserLicensesResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn run_named_query_continue<T: transport::Transport>(
    transport: &T,
    request: &RunNamedQueryContinue
) -> Result<RunNamedQueryContinueResponse, transport::Error> {
    transport::request(transport, request).await
}

pub async fn get_guis<T: transport::Transport>(
    transport: &T,
    request: &GetGUIs
) -> Result<GetGUIsResponse, transport::Error> {
    transport::request(transport, request).await
}
