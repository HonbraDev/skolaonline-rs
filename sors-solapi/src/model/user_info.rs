use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(rename = "userUID")]
    pub user_uid: String,
    #[serde(rename = "organizationID")]
    pub organization_id: String,
    #[serde(rename = "personID")]
    pub person_id: String,
    pub campaign_category_code: String,
    pub class: Class,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
    pub detail_name: String,
    pub persons_initials: String,
    pub school_organization_name: String,
    pub school_type: String,
    pub user_type: String,
    pub user_type_text: String,
    pub study_year: u32,
    pub children: Vec<Value>,
    pub class_teacher_classes: Vec<Value>,
    pub deputy_class_teacher_classes: Vec<Value>,
    pub install_type: String,
    pub language: String,
    pub school_year_id: String,
    pub enabled_modules: Vec<EnabledModule>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: String,
    pub abbrev: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnabledModule {
    pub module: EnabledModuleModule,
    pub rights: Vec<Value>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnabledModuleModule {
    pub module_id: String,
    pub module_name: String,
}
