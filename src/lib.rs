pub const VERSION: &str = "1.0.0";

pub const CANONICAL_LAYERS: &[&str] = &[
    "able", "are", "dio", "fak", "gsas",
    "icae", "icl", "jib", "poc", "zt-aas",
];

pub fn is_valid_layer(name: &str) -> bool {
    CANONICAL_LAYERS.contains(&name)
}