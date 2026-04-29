pub const VERSION: &str = "1.0.0";

pub const CANONICAL_CORE_LAYERS: &[&str] = &[
    "dio", "zt-aas", "icae", "poc", "fak", "are", "jib", "icl", "gsas", "able",
];

pub const CANONICAL_EXTENSION_LAYERS: &[&str] = &[
    // Future unlocked extension layers go here.
];

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
}
