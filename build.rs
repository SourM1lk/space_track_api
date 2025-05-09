use std::env;
use std::fs;
use std::path::PathBuf;

/// (controller, class, fn_name, RustReturnType)
const CLASSES: &[(&str, &str, &str, &str)] = &[
    // basicspacedata
    (
        "basicspacedata",
        "satcat",
        "satcat",
        "crate::models::SatCatEntry",
    ),
    ("basicspacedata", "gp", "gp", "crate::models::GpEntry"),
    (
        "basicspacedata",
        "gp_history",
        "gp_history",
        "crate::models::GpHistoryEntry",
    ),
    (
        "basicspacedata",
        "boxscore",
        "boxscore",
        "crate::models::BoxScore",
    ),
    (
        "basicspacedata",
        "decay",
        "decay",
        "crate::models::DecayEntry",
    ),
    // add more here…
];

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("class_fns.rs");

    let mut code = String::new();
    code.push_str("impl crate::client::SpaceTrackClient {\n");

    for (controller, class, fn_name, ty) in CLASSES {
        code.push_str(&format!(
            "    pub fn {fname}<'a>(&'a self) -> crate::query::QueryBuilder<'a, {ty}> {{\n",
            fname = fn_name,
            ty = ty,
        ));
        code.push_str(&format!(
            "        self.query_builder::<{ty}>(\"{controller}\", \"{class}\")\n",
            controller = controller,
            class = class,
            ty = ty,
        ));
        code.push_str("    }\n");
    }

    code.push_str("}\n");

    fs::write(dest, code).unwrap();
}
