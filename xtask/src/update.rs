use convert_case::{Case, Casing};
use regex::Regex;
use std::fs;

fn match_case(weight_name: &str, svg: &str) -> String {
    format!(
        "IconWeight::{} => view!{{ {} }}.into_view()",
        weight_name.to_case(Case::UpperCamel),
        svg
    )
}

fn template(icon_name: &str, icon_weights: Vec<(String, String)>) -> String {
    let component_name = icon_name.to_case(Case::UpperCamel);
    format!(
        r#"/// GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[component]
pub fn {component_name}(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>
) -> impl IntoView {{
    let body = move || {{
        match weight.get() {{
            {}
        }}
    }};

    let transform = move || if mirrored.get() {{ "scale(-1, 1)" }} else {{ "" }};

    view! {{
        <svg 
            xmlns="http://www.w3.org/2000/svg" 
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {{body}}
        </svg>
    }}
}}"#,
        icon_weights
            .iter()
            .map(|w| match_case(&w.0, &w.1))
            .collect::<Vec<_>>()
            .join(",\n")
    )
}

const OUTPUT_DIR: &str = "src/icons";
const ASSETS_DIR: &str = "core/assets";

pub fn run() {
    let svg_tag_regex = Regex::new(r"<svg.*?>").unwrap();
    let svg_closing_tag_regex = Regex::new(r"</svg>").unwrap();
    // Clean up the icons folder
    let _ = fs::remove_dir_all(OUTPUT_DIR);
    fs::write("src/lib.rs", "").unwrap();
    fs::create_dir(OUTPUT_DIR).unwrap();

    // Get a list of all the icon weights
    let weights: Vec<_> = fs::read_dir(ASSETS_DIR)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    let regular_icons = fs::read_dir(format!("{ASSETS_DIR}/regular")).unwrap();
    let mut mod_content = String::new();
    for entry in regular_icons {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let file_name = entry.file_name().into_string().unwrap();
            let icon_name = file_name.strip_suffix(".svg").unwrap().to_string();

            let icon_weights: Vec<_> = weights
                .iter()
                .map(|weight| {
                    let file_name = if weight == "regular" {
                        format!("{icon_name}.svg")
                    } else {
                        format!("{icon_name}-{weight}.svg")
                    };
                    let svg =
                        fs::read_to_string(format!("{ASSETS_DIR}/{weight}/{file_name}")).unwrap();
                    let svg = svg_tag_regex.replace(&svg, "");
                    let svg = svg_closing_tag_regex.replace(&svg, "");
                    (weight.to_string(), svg.to_string())
                })
                .collect();
            fs::write(
                format!("{OUTPUT_DIR}/{}.rs", icon_name.to_case(Case::Snake)),
                template(&icon_name, icon_weights),
            )
            .unwrap();

            mod_content.push_str(&format!(
                "mod {name};\npub use {name}::*;\n",
                name = icon_name.to_case(Case::Snake)
            ));
        };
    }

    fs::write(format!("{OUTPUT_DIR}/mod.rs"), mod_content).unwrap();
    fs::write(
        "src/lib.rs",
        format!(
            r#"mod icons;
pub use icons::*;
            
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconWeight {{
    {}
}}
"#,
            weights
                .iter()
                .map(|w| w.to_case(Case::UpperCamel))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .unwrap();
}
