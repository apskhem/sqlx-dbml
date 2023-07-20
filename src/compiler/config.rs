use std::ffi::OsString;

#[derive(Debug)]
pub enum EnumTypeCase {
  LowerCase,
  SnakeCase,
  UpperCase,
  ScreamingSnakeCase,
  KebabCase,
  CamelCase,
  PascalCase,
}

#[derive(Debug)]
pub enum EnumType {
  String(Option<EnumTypeCase>),
  Integer,
}

#[derive(Debug)]
pub enum StructVisibility {
  Public,
  Private
}

/// Database entity target.
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
  // MySQL,
  // SQLite,
  // MSSQL,
  Postgres,
}

#[derive(Debug)]
/// Configuration options for the code generation.
pub struct Config {
  /// Input file path.
  pub in_path: OsString,
  /// Output file path (optional). The default output path is `OUT_DIR`.
  pub out_path: OsString,
  /// Database entity target.
  pub target: Target,
  /// Enum type for storing a value. It can be either `String` and `Interger`.
  /// The `String` type requires to specify the renaming case.
  pub enum_type: EnumType,
  /// The list of injecting marcros into geenrated structs.
  pub table_macros: Vec<String>,
  /// The list of injecting marcros into geenrated enums.
  pub enum_macros: Vec<String>,
  /// The visibility of fields in generated structs.
  pub struct_visibility: StructVisibility,
  /// Output update schemas with optional all fields by default, entity's primary key field is omitted.
  pub is_with_update_model: bool,
  /// Ouput create schemas with optional default fields.
  /// The entity's primary key field is included when it is not being auto generated.
  pub is_with_create_model: bool,
  /// Forcing `is_with_create_schema` wether it should include primary key field or not.
  pub is_create_model_primary_key_included: Option<bool>,
  /// Config `is_with_update_schema` to include primary key field.
  pub is_update_model_primary_key_included: bool,
  /// Generate the `into_args_pair` method for every model.
  pub is_with_into_args_pair_method: bool
}

impl Default for Config {
  fn default() -> Self {
    Self {
      in_path: OsString::from(""),
      out_path: OsString::from(""),
      target: Target::Postgres,
      enum_type: EnumType::String(None),
      table_macros: vec![],
      enum_macros: vec![],
      struct_visibility: StructVisibility::Public,
      is_with_update_model: false,
      is_with_create_model: false,
      is_create_model_primary_key_included: None,
      is_update_model_primary_key_included: false,
      is_with_into_args_pair_method: false
    }
  }
}

impl Config {
  pub fn validate(&self) -> Option<&str> {
    if self.in_path.is_empty() {
      Some("in_path is not set")
    } else if self.out_path.is_empty() {
      Some("out_path is not set")
    } else {
      None
    }
  }
}
