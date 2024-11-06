/*
 * Cube.js
 *
 * Cube.js Swagger Schema
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// V1CubeMetaType : Type of cube

/// Type of cube
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum V1CubeMetaType {
    #[serde(rename = "cube")]
    Cube,
    #[serde(rename = "view")]
    View,
}

impl ToString for V1CubeMetaType {
    fn to_string(&self) -> String {
        match self {
            Self::Cube => String::from("cube"),
            Self::View => String::from("view"),
        }
    }
}

impl Default for V1CubeMetaType {
    fn default() -> V1CubeMetaType {
        Self::Cube
    }
}
