use crate::sbml::import::{child_tags, read_unique_child, SBML_QUAL};
use roxmltree::Node;

/// Approximate representation of an SBML specie. Note that only ID is required, all other
/// properties are optional.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SBMLSpecie {
    pub id: String,
    pub compartment: Option<String>,
    pub name: Option<String>,
    pub max_level: Option<u32>,
    pub is_constant: bool,
}

pub fn read_species(model: Node) -> Result<Vec<SBMLSpecie>, String> {
    let mut result = Vec::new();

    let list = read_unique_child(model, (SBML_QUAL, "listOfQualitativeSpecies"))?;

    let species = child_tags(list, (SBML_QUAL, "qualitativeSpecies"));

    for specie in species {
        if let Some(id) = specie.attribute((SBML_QUAL, "id")) {
            let compartment = specie
                .attribute((SBML_QUAL, "compartment"))
                .map(|s| s.to_string());
            let name = specie.attribute((SBML_QUAL, "name")).map(|s| s.to_string());
            let max_level = specie.attribute((SBML_QUAL, "maxLevel"));
            let max_level = if let Some(max_level) = max_level {
                let value = max_level.parse::<u32>();
                if value.is_err() {
                    return Err(format!("Invalid maxLevel value: {}", max_level));
                } else {
                    Some(value.unwrap())
                }
            } else {
                None
            };
            let is_constant = specie
                .attribute((SBML_QUAL, "constant"))
                .map(|s| s == "true");
            result.push(SBMLSpecie {
                id: id.to_string(),
                is_constant: is_constant.unwrap_or(false),
                compartment,
                name,
                max_level,
            });
        } else {
            return Err(format!("Qualitative specie with a missing ID."));
        }
    }

    Ok(result)
}
