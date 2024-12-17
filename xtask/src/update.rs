use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::{fs, process};

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
version = "0.6.0"
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
leptos = "0.7.1"

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
    for feature in features.keys() {
        template.push_str(&format!("{feature} = []\n"));
    }

    template
}

fn icon_template(
    icon_name: &str,
    icon_weights: impl Iterator<Item = (String, String)>,
) -> TokenStream {
    let component_ident = format_ident!("{}", icon_name.to_case(Case::UpperSnake));
    let weights = icon_weights.map(|w| w.1);

    quote! {
        //! GENERATED FILE
        pub const #component_ident: &crate::IconWeightData = &crate::IconWeightData([#(#weights),*]);
    }
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

    let uncategorized = vec!["uncategorized".into()];

    // Clean up the icons folder
    let _ = fs::remove_dir_all(OUTPUT_DIR);
    fs::write("src/lib.rs", "").unwrap();
    fs::create_dir(OUTPUT_DIR).unwrap();

    // Get a list of all the icon weights
    let mut weights: Vec<_> = fs::read_dir(ASSETS_DIR)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    // Sort the weights so their ordering is stable.
    weights.sort_unstable();

    let regular_icons = fs::read_dir(format!("{ASSETS_DIR}/regular")).unwrap();

    let mut file_names: Vec<_> = regular_icons
        .into_iter()
        .filter_map(|e| {
            let entry = e.unwrap();
            if entry.path().is_file() {
                Some(entry.file_name().into_string().unwrap())
            } else {
                None
            }
        })
        .collect();

    // We'll also sort the file names so each generation run has a
    // stable order. This should improve `src/mod.rs` diffs.
    file_names.sort_unstable();

    let mut mod_content = Vec::new();
    for file_name in file_names {
        let icon_name = file_name.strip_suffix(".svg").unwrap().to_string();

        //derive the feature set string for this icon from its mappings.
        //If we haven't been able to match the icon's category, assign in to 'Uncategorized'
        let features = icon_categories.get(&icon_name).unwrap_or(&uncategorized);

        let icon_weights = weights.iter().map(|weight| {
            let file_name = if weight == "regular" {
                format!("{icon_name}.svg")
            } else {
                format!("{icon_name}-{weight}.svg")
            };
            let svg = fs::read_to_string(format!("{ASSETS_DIR}/{weight}/{file_name}")).unwrap();
            let svg = svg_tag_regex.replace(&svg, "");
            let svg = svg_closing_tag_regex.replace(&svg, "");
            (weight.to_string(), svg.to_string())
        });

        let file = icon_template(&icon_name, icon_weights);

        fs::write(
            format!("{OUTPUT_DIR}/{}.rs", icon_name.to_case(Case::Snake)),
            file.to_string(),
        )
        .unwrap();

        let mod_name = format_ident!("{}", icon_name.to_case(Case::Snake));
        mod_content.push(quote! {
            #[cfg(any(#(feature = #features),*))]
            #[doc(hidden)]
            mod #mod_name;

            #[cfg(any(#(feature = #features),*))]
            #[doc(hidden)]
            pub use #mod_name::*;
        });
    }

    let module = quote! { #(#mod_content)* }.to_string();
    fs::write(format!("{OUTPUT_DIR}/mod.rs"), module).unwrap();

    let weight_variants: Vec<_> = weights
        .iter()
        .map(|w| format_ident!("{}", w.to_case(Case::UpperCamel)))
        .collect();

    let weight_len = weight_variants.len();
    let weight_indeces = weight_variants.iter().enumerate().map(|(i, v)| {
        quote! { IconWeight::#v => self.0[#i] }
    });

    let lib = quote! {
        //! Phosphor is a flexible icon family for interfaces, diagrams,
        //! presentations — whatever, really.
        //! You can explore the available icons at [phosphoricons.com](https://phosphoricons.com).
        //!
        //! ```
        //! use leptos::prelude::*;
        //! use phosphor_leptos::{Icon, IconWeight, HORSE, HEART, CUBE};
        //!
        //! #[component]
        //! fn MyComponent() -> impl IntoView {
        //!     view! {
        //!         <Icon icon=HORSE />
        //!         <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />
        //!         <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />
        //!     }
        //! }
        //! ```
        use leptos::prelude::*;
        use leptos::text_prop::TextProp;

        mod icons;
        pub use icons::*;

        /// An icon's weight or style.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum IconWeight {
            #(#weight_variants),*
        }

        /// The SVG path data for all weights of a particular icon.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct IconWeightData([&'static str; #weight_len]);

        impl IconWeightData {
            /// Retrieve the SVG paths for the given weight.
            ///
            /// The returned string slice contains raw path elements.
            /// To render them manually, you'll need to provide them to
            /// an SVG component's `inner_html` property.
            ///
            /// ```
            /// # use leptos::prelude::*;
            /// # #[component]
            /// # fn MyComponent() -> impl IntoView {
            /// use phosphor_leptos::{ACORN, IconWeight};
            ///
            /// let raw_html = ACORN.get(IconWeight::Regular);
            /// view! {
            ///     <svg inner_html=raw_html />
            /// }
            /// # }
            /// ```
            pub fn get(&self, weight: IconWeight) -> &'static str {
                match weight {
                    #(#weight_indeces),*
                }
            }
        }

        /// A convenient alias for passing around references to [IconWeightData].
        ///
        /// While [IconWeightData] is `Copy`, it's not particularly beneficial to pass
        /// all those bytes around (48 bytes on WASM, 96 bytes on 64 bit systems).
        pub type IconData = &'static IconWeightData;

        /// A thin wrapper around `<svg />` for displaying Phosphor icons.
        ///
        /// ```
        /// use leptos::prelude::*;
        /// use phosphor_leptos::{Icon, IconWeight, HORSE, HEART, CUBE};
        ///
        /// #[component]
        /// fn MyComponent() -> impl IntoView {
        ///     view! {
        ///         <Icon icon=HORSE />
        ///         <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />
        ///         <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />
        ///     }
        /// }
        /// ```
        #[component]
        pub fn Icon(
            /// The icon data to display.
            icon: IconData,

            /// Icon weight/style. This can also be used, for example, to "toggle" an icon's state:
            /// a rating component could use Stars with [IconWeight::Regular] to denote an empty star,
            /// and [IconWeight::Fill] to denote a filled star.
            #[prop(into, default = Signal::from (IconWeight::Regular))] weight: Signal<
                IconWeight,
            >,

            /// Icon height & width. As with standard React elements,
            /// this can be a number, or a string with units in
            /// `px`, `%`, `em`, `rem`, `pt`, `cm`, `mm`, `in`.
            #[prop(into, default = TextProp::from("1em"))] size: TextProp,

            /// Icon stroke/fill color.
            ///
            /// This can be any CSS color string, including
            /// `hex`, `rgb`, `rgba`, `hsl`, `hsla`, named colors,
            /// or the special `currentColor` variable.
            ///
            #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,

            /// Flip the icon horizontally.
            ///
            /// This can be useful in RTL languages where normal
            /// icon orientation is not appropriate.
            #[prop(into, default = Signal::from(false))] mirrored: Signal<bool>,

            /// The HTML ID of the underlying SVG element.
            #[prop(into, optional)] id: TextProp,

            /// The CSS class property of the underlying SVG element.
            #[prop(into, optional)] class: TextProp,
        ) -> impl IntoView {
            let html = move || icon.get(weight.get());
            let transform = move || mirrored.get().then_some("scale(-1, 1)");
            let height = size.clone();

            view! {
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width=move || size.get()
                    height=move || height.get()
                    fill=color
                    transform=transform
                    viewBox="0 0 256 256"
                    id=move || id.get().map(|id| id.get())
                    class=move || class.get().map(|cls| cls.get())
                    inner_html=html
                />
            }
        }
    };

    fs::write("src/lib.rs", lib.to_string()).expect("Error writing lib file");

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
