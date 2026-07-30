pub const VERSION: &str = "1.0.0";

pub const CANONICAL_CORE_LAYERS: &[&str] = &[
    "dio", "zt-aas", "icae", "poc", "fak", "are", "jib", "icl", "gsas", "able",
];

pub const CANONICAL_EXTENSION_LAYERS: &[&str] = &[
    // Future unlocked extension layers go here.
];

const _: () = assert_extension_layers_do_not_redefine_core();

const fn assert_extension_layers_do_not_redefine_core() {
    let mut extension_index = 0;
    while extension_index < CANONICAL_EXTENSION_LAYERS.len() {
        let extension_layer = CANONICAL_EXTENSION_LAYERS[extension_index];
        let mut core_index = 0;
        while core_index < CANONICAL_CORE_LAYERS.len() {
            if layer_names_eq(extension_layer, CANONICAL_CORE_LAYERS[core_index]) {
                panic!("Core layers must not be redefined in CANONICAL_EXTENSION_LAYERS");
            }
            core_index += 1;
        }
        extension_index += 1;
    }
}

const fn layer_names_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();

    if left.len() != right.len() {
        return false;
    }

    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }

    true
}

#[deprecated(
    note = "Use CANONICAL_CORE_LAYERS for mandatory Core or canonical_layers() for Core + extensions"
)]
pub const CANONICAL_LAYERS: &[&str] = CANONICAL_CORE_LAYERS;

pub fn canonical_layers() -> Vec<&'static str> {
    CANONICAL_CORE_LAYERS
        .iter()
        .chain(CANONICAL_EXTENSION_LAYERS.iter())
        .copied()
        .collect()
}

pub fn is_valid_layer(name: &str) -> bool {
    CANONICAL_CORE_LAYERS.contains(&name) || CANONICAL_EXTENSION_LAYERS.contains(&name)
}

pub fn is_core_layer(name: &str) -> bool {
    CANONICAL_CORE_LAYERS.contains(&name)
}

pub fn is_extension_layer(name: &str) -> bool {
    CANONICAL_EXTENSION_LAYERS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_MESH_NAME_PATTERN: &str = "^[A-Za-z0-9][A-Za-z0-9_.-]{0,63}$";
    const LOG_FILE_LEXICAL_GUARDS: &[&str] = &[
        "^(?:[\\\\/]|[A-Za-z]:)",
        "(?:^|[\\\\/])\\.\\.(?:[\\\\/]|$)",
        "^(?:\\.(?:[\\\\/]+|$))+$",
    ];

    fn public_config_schema() -> serde_json::Value {
        serde_json::from_str(include_str!("../schemas/config.schema.json"))
            .expect("config schema should parse")
    }

    #[test]
    fn canonical_layers_are_derived_from_core_and_extensions() {
        let mut expected = CANONICAL_CORE_LAYERS.to_vec();
        expected.extend_from_slice(CANONICAL_EXTENSION_LAYERS);

        assert_eq!(canonical_layers(), expected);
    }

    #[test]
    fn core_layers_are_valid_core_layers() {
        for layer in CANONICAL_CORE_LAYERS {
            assert!(is_valid_layer(layer));
            assert!(is_core_layer(layer));
            assert!(!is_extension_layer(layer));
        }
    }

    #[test]
    fn extension_layers_do_not_redefine_core_layers() {
        for extension_layer in CANONICAL_EXTENSION_LAYERS {
            assert!(
                !CANONICAL_CORE_LAYERS.contains(extension_layer),
                "{extension_layer} is already a mandatory Core layer"
            );
        }
    }

    #[test]
    fn public_config_schema_accepts_only_the_supported_version() {
        let schema = public_config_schema();
        let version = &schema["properties"]["version"];

        assert_eq!(version["const"], serde_json::json!(VERSION));
        assert!(version.get("pattern").is_none());
    }

    #[test]
    fn public_config_schema_uses_the_runtime_mesh_identity_grammar() {
        let schema = public_config_schema();
        let patterns = schema["properties"]["meshes"]["patternProperties"]
            .as_object()
            .expect("mesh patternProperties should be an object");

        assert_eq!(patterns.len(), 1);
        assert!(patterns.contains_key(PUBLIC_MESH_NAME_PATTERN));
    }

    #[test]
    fn public_config_schema_rejects_lexically_unsafe_log_paths() {
        let schema = public_config_schema();
        let log_file = &schema["properties"]["options"]["properties"]["log_file"];

        assert_eq!(log_file["minLength"], serde_json::json!(1));
        let guards = log_file["allOf"]
            .as_array()
            .expect("log_file allOf should be an array")
            .iter()
            .map(|guard| {
                guard["not"]["pattern"]
                    .as_str()
                    .expect("log path guard should be a pattern")
            })
            .collect::<Vec<_>>();
        assert_eq!(guards, LOG_FILE_LEXICAL_GUARDS);
    }

    #[test]
    fn public_config_schema_enforces_all_or_none_core_layers() {
        let schema = public_config_schema();

        let meshes = &schema["properties"]["meshes"];
        assert_eq!(meshes["additionalProperties"], serde_json::json!(false));

        let layer_schema =
            &meshes["patternProperties"][PUBLIC_MESH_NAME_PATTERN]["properties"]["layers"];
        assert_eq!(
            layer_schema["minItems"],
            serde_json::json!(CANONICAL_CORE_LAYERS.len())
        );
        assert_eq!(
            layer_schema["maxItems"],
            serde_json::json!(CANONICAL_CORE_LAYERS.len())
        );
        assert_eq!(layer_schema["uniqueItems"], serde_json::json!(true));

        let schema_layers = layer_schema["items"]["enum"]
            .as_array()
            .expect("layer enum should be an array")
            .iter()
            .map(|value| value.as_str().expect("layer enum values should be strings"))
            .collect::<Vec<_>>();
        assert_eq!(schema_layers, CANONICAL_CORE_LAYERS);
    }
}
