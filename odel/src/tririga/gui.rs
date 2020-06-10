
    use super::dto::DataType;
    use std::io::{Read, Write};
    use yaserde::{YaDeserialize, YaSerialize};
    use yaserde_derive::{YaDeserialize, YaSerialize};
    use super::transport::Validate;

    //use   http://dto.ws.tririga.com;
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "GuiRow")]
    pub struct GuiRow {
        #[yaserde(prefix = "gui", rename = "fields")]
        pub fields: Option<ArrayOfField>,

        #[yaserde(prefix = "gui", rename = "recordId")]
        pub record_id: Option<i64>,
    }

    impl Validate for GuiRow {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "ArrayOfTab")]
    pub struct ArrayOfTab {
        #[yaserde(prefix = "gui", rename = "Tab")]
        pub tab: Vec<Tab>,
    }

    impl Validate for ArrayOfTab {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "Tab")]
    pub struct Tab {
        #[yaserde(prefix = "gui", rename = "id")]
        pub id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "instruction")]
        pub instruction: Option<String>,

        #[yaserde(prefix = "gui", rename = "label")]
        pub label: Option<String>,

        #[yaserde(prefix = "gui", rename = "name")]
        pub name: Option<String>,

        #[yaserde(prefix = "gui", rename = "sections")]
        pub sections: Option<ArrayOfSection>,

        #[yaserde(prefix = "gui", rename = "tabInformation")]
        pub tab_information: Option<String>,

        #[yaserde(prefix = "gui", rename = "url")]
        pub url: Option<String>,

        #[yaserde(prefix = "gui", rename = "visible")]
        pub visible: Option<bool>,
    }

    impl Validate for Tab {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "Section")]
    pub struct Section {
        #[yaserde(prefix = "gui", rename = "fields")]
        pub fields: Option<ArrayOfField>,

        #[yaserde(prefix = "gui", rename = "guiSectionId")]
        pub gui_section_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "guiSectionName")]
        pub gui_section_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "id")]
        pub id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "label")]
        pub label: Option<String>,

        #[yaserde(prefix = "gui", rename = "name")]
        pub name: Option<String>,

        #[yaserde(prefix = "gui", rename = "readOnly")]
        pub read_only: Option<bool>,

        #[yaserde(prefix = "gui", rename = "refModuleId")]
        pub ref_module_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "refObjectTypeId")]
        pub ref_object_type_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "rows")]
        pub rows: Option<ArrayOfGuiRow>,

        #[yaserde(prefix = "gui", rename = "type")]
        pub _type: Option<String>,

        #[yaserde(prefix = "gui", rename = "typeId")]
        pub type_id: Option<i32>,

        #[yaserde(prefix = "gui", rename = "visible")]
        pub visible: Option<bool>,
    }

    impl Validate for Section {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "ArrayOfSection")]
    pub struct ArrayOfSection {
        #[yaserde(prefix = "gui", rename = "Section")]
        pub section: Vec<Section>,
    }

    impl Validate for ArrayOfSection {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "ArrayOfField")]
    pub struct ArrayOfField {
        #[yaserde(prefix = "gui", rename = "Field")]
        pub field: Vec<Field>,
    }

    impl Validate for ArrayOfField {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "ArrayOfGUI")]
    pub struct ArrayOfGUI {
        #[yaserde(prefix = "gui", rename = "GUI")]
        pub gui: Vec<Gui>,
    }

    impl Validate for ArrayOfGUI {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "GUI")]
    pub struct Gui {
        #[yaserde(prefix = "gui", rename = "childRecordId")]
        pub child_record_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "id")]
        pub id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "label")]
        pub label: Option<String>,

        #[yaserde(prefix = "gui", rename = "moduleId")]
        pub module_id: Option<i32>,

        #[yaserde(prefix = "gui", rename = "moduleName")]
        pub module_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "name")]
        pub name: Option<String>,

        #[yaserde(prefix = "gui", rename = "objectTypeId")]
        pub object_type_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "objectTypeName")]
        pub object_type_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "popupId")]
        pub popup_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "recordId")]
        pub record_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "recordName")]
        pub record_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "saveOnPopup")]
        pub save_on_popup: Option<bool>,

        #[yaserde(prefix = "gui", rename = "singleTab")]
        pub single_tab: Option<bool>,

        #[yaserde(prefix = "gui", rename = "state")]
        pub state: Option<String>,

        #[yaserde(prefix = "gui", rename = "tabs")]
        pub tabs: Option<ArrayOfTab>,

        #[yaserde(prefix = "gui", rename = "type")]
        pub _type: Option<String>,
    }

    impl Validate for Gui {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "ArrayOfGuiRow")]
    pub struct ArrayOfGuiRow {
        #[yaserde(prefix = "gui", rename = "GuiRow")]
        pub gui_row: Vec<GuiRow>,
    }

    impl Validate for ArrayOfGuiRow {}


    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "gui", namespace = "gui: http://gui.dto.ws.tririga.com", rename = "Field")]
    pub struct Field {
        #[yaserde(prefix = "gui", rename = "dataType")]
        pub data_type: Option<DataType>,

        #[yaserde(prefix = "gui", rename = "displayValue")]
        pub display_value: Option<String>,

        #[yaserde(prefix = "gui", rename = "guiRootClassificationId")]
        pub gui_root_classification_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "guiRootClassificationName")]
        pub gui_root_classification_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "guiSectionId")]
        pub gui_section_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "guiSectionName")]
        pub gui_section_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "label")]
        pub label: Option<String>,

        #[yaserde(prefix = "gui", rename = "listId")]
        pub list_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "listModuleId")]
        pub list_module_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "liveLink")]
        pub live_link: Option<bool>,

        #[yaserde(prefix = "gui", rename = "locatorField")]
        pub locator_field: Option<bool>,

        #[yaserde(prefix = "gui", rename = "locatorModuleId")]
        pub locator_module_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "mobileField")]
        pub mobile_field: Option<bool>,

        #[yaserde(prefix = "gui", rename = "mobileFieldSeq")]
        pub mobile_field_seq: Option<i64>,

        #[yaserde(prefix = "gui", rename = "name")]
        pub name: Option<String>,

        #[yaserde(prefix = "gui", rename = "readOnly")]
        pub read_only: Option<bool>,

        #[yaserde(prefix = "gui", rename = "refObjectTypeId")]
        pub ref_object_type_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "refSpecId")]
        pub ref_spec_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "required")]
        pub required: Option<bool>,

        #[yaserde(prefix = "gui", rename = "rootClassification")]
        pub root_classification: Option<String>,

        #[yaserde(prefix = "gui", rename = "rootClassificationId")]
        pub root_classification_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "sectionName")]
        pub section_name: Option<String>,

        #[yaserde(prefix = "gui", rename = "specId")]
        pub spec_id: Option<i64>,

        #[yaserde(prefix = "gui", rename = "type")]
        pub _type: Option<String>,

        #[yaserde(prefix = "gui", rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,

        #[yaserde(prefix = "gui", rename = "unitOfMeasureType")]
        pub unit_of_measure_type: Option<i64>,

        #[yaserde(prefix = "gui", rename = "value")]
        pub value: Option<String>,
    }

    impl Validate for Field {}

