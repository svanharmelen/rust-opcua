mod base_constants;
mod enum_type;
#[cfg(feature = "codegen")]
mod gen;
mod loader;
mod structure;

pub use base_constants::*;
pub use enum_type::{EnumType, EnumValue};
pub use gen::{CodeGenItemConfig, CodeGenerator, GeneratedItem, ItemDefinition};
pub use loader::{BsdTypeLoader, LoadedType, LoadedTypes};
use opcua_xml::load_bsd_file;
pub use structure::{StructureField, StructureFieldType, StructuredType};

use crate::{CodeGenError, TypeCodeGenTarget};

pub fn generate_types(target: &TypeCodeGenTarget) -> Result<Vec<GeneratedItem>, CodeGenError> {
    println!("Loading types from {}", target.file_path);
    let data = std::fs::read_to_string(&target.file_path)
        .map_err(|e| CodeGenError::io(&format!("Failed to read file {}", target.file_path), e))?;
    let type_dictionary = load_bsd_file(&data)?;
    println!(
        "Found {} raw elements in the type dictionary.",
        type_dictionary.elements.len()
    );
    let type_loader = BsdTypeLoader::new(
        target
            .ignore
            .iter()
            .cloned()
            .chain(base_ignored_types().into_iter())
            .collect(),
        base_native_type_mappings(),
        type_dictionary,
    )?;
    let types = type_loader.from_bsd()?;
    println!("Generated code for {} types", types.len());

    let mut types_import_map = basic_types_import_map();
    for (k, v) in &target.types_import_map {
        types_import_map.insert(k.clone(), v.clone());
    }

    let generator = CodeGenerator::new(
        types_import_map,
        types,
        target.default_excluded.clone(),
        CodeGenItemConfig {
            enums_single_file: target.enums_single_file,
            structs_single_file: target.structs_single_file,
        },
    );

    generator.generate_types()
}
