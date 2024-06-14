use convert_case::{Case, Casing};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::{fs, process};

fn match_case(weight_name: &str, svg: &str) -> String {
    format!(
        "IconWeight::{} => view!{{ {} }}.into_view()",
        weight_name.to_case(Case::UpperCamel),
        svg
    )
}

fn extract_categories(input: &str) -> (HashMap<String, Vec<String>>, BTreeMap<String, ()>) {
    let mut icon_categories: HashMap<String, Vec<String>> = HashMap::new();
    let mut categories_set: BTreeMap<String, ()> = BTreeMap::new();

    let re = Regex::new(r#"(?m)^\s*name:\s*"(.+)",\n.*\n\s*categories:\s*\[([^]]+)\]"#).unwrap();

    for cap in re.captures_iter(input) {
        let name = cap[1].to_string();
        let has_categories = cap[2]
            .split(',')
            .filter(|category| !category.trim().is_empty())
            .map(|category| {
                let value = category
                    .trim()
                    .split('.')
                    .nth(1)
                    .unwrap()
                    .to_lowercase()
                    .to_string();
                categories_set.insert(value.clone(), ());
                value
            })
            .collect::<Vec<String>>();

        icon_categories.insert(name, has_categories);
    }
    // Insert the Uncategorized category for icons that are not in the TS export file
    categories_set.insert("uncategorized".to_string(), ());
    (icon_categories, categories_set)
}
fn cargo_template(features: &BTreeMap<String, ()>) -> String {
    let mut template = r#"# GENERATED FILE!
# Edit xtask/src/update.rs to maintain this file

[package]
name = "phosphor-leptos"
version = "0.5.0"
description = "phosphor icons for leptos"
authors = ["Søren H. Hansen"]
readme = "README.md"
repository = "https://github.com/SorenHolstHansen/phosphor-leptos"
keywords = ["icons", "leptos", "phosphor"]
edition = "2021"
license = "MIT"
exclude = ["/core"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
leptos = "0.6"

[workspace]
members = ["xtask"]

[features]
default = ["all"]
"#
    .to_string();

    // Add in the "all" feature
    template.push_str(&format!(
        "all = [\n{}\n]\n",
        features
            .iter()
            .map(|(feature, _)| format!("\t\"{feature}\""))
            .collect::<Vec<_>>()
            .join(",\n"),
    ));

    // now add the rest, read from the icon_categories
    for (feature, _) in features {
        template.push_str(&format!("{feature} = []\n"));
    }

    template
}

fn icon_template(
    icon_name: &str,
    icon_weights: Vec<(String, String)>,
    features: &String,
) -> String {
    let component_name = icon_name.to_case(Case::UpperCamel);
    format!(
        r#"//! GENERATED FILE

use leptos::*;
use crate::IconWeight;

#[cfg(any({features}))]
#[component]
pub fn {component_name}(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<IconWeight>,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {{
    let body = Signal::derive(move || {{
        match weight.get() {{
            {}
        }}
    }});

    let transform = move || if mirrored.get() {{ "scale(-1, 1)" }} else {{ "" }};
    let height = size.clone();

    view! {{
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=height
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=id
            class=class
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
const TYPESCRIPT_EXPORT_FILE: &str = "core/src/icons.ts";

pub fn run() {
    let svg_tag_regex = Regex::new(r"<svg.*?>").unwrap();
    let svg_closing_tag_regex = Regex::new(r"</svg>").unwrap();

    // Extract the categories from the typescript export file
    let (icon_categories, categories_set) =
        extract_categories(fs::read_to_string(TYPESCRIPT_EXPORT_FILE).unwrap().as_str());

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

            //derive the feature set string for this icon from its mappings.
            //If we haven't been able to match the icon's category, assign in to 'Uncategorized'
            let features = icon_categories
                .get(&icon_name)
                .unwrap_or(&vec!["uncategorized".to_string()])
                .iter()
                .map(|category| format!(r#"feature ="{category}""#))
                .collect::<Vec<_>>();
            let features = features.join(", ");

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
                icon_template(&icon_name, icon_weights, &features),
            )
            .unwrap();

            mod_content.push_str(&format!(
                "#[cfg(any({features}))]\nmod {name};\n#[cfg(any({features}))]\npub use {name}::*;\n",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // Write out the newly generated cargo file
    fs::write("Cargo.toml", cargo_template(&categories_set)).unwrap();

    process::Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("Error running cargo fmt");
    process::Command::new("leptosfmt")
        .arg("src")
        .status()
        .expect("Error running leptosfmt");
}
