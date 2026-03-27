use serde::Deserialize;

use crate::models::{Category, Form, Lemma, Tag};

/// Raw dump struct matching what build.rs generates.
#[derive(Deserialize)]
struct DbDump {
    lemmas: Vec<Lemma>,
    forms: Vec<Form>,
    categories: Vec<FlatCategory>,
    tags: Vec<Tag>,
}

#[derive(Deserialize)]
struct FlatCategory {
    id: i64,
    parent_id: Option<i64>,
    code: Option<String>,
    name_ru: String,
    name_en: String,
    name_gr: Option<String>,
    depth: i32,
    sort_order: i32,
    description: Option<String>,
}

/// The embedded JSON generated at build time by build.rs.
static DB_JSON: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/greek_data.json"));

pub struct LoadedData {
    pub forms: Vec<Form>,
    pub lemmas: Vec<Lemma>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

/// Parse the embedded JSON and return all data.
/// Called once at app startup.
pub fn load() -> LoadedData {
    let dump: DbDump = serde_json::from_slice(DB_JSON).expect("invalid greek_data.json");

    let categories = build_category_tree(dump.categories);

    LoadedData { forms: dump.forms, lemmas: dump.lemmas, categories, tags: dump.tags }
}

/// Assemble a flat list of FlatCategory into a nested tree, returning
/// only the top-level (depth 0 / root) nodes with children populated.
/// We keep the structure flat (Vec<Category>) sorted by depth+sort_order
/// so UI components can easily render a tree.
fn build_category_tree(flat: Vec<FlatCategory>) -> Vec<Category> {
    use std::collections::HashMap;

    let mut map: HashMap<i64, Category> = flat
        .iter()
        .map(|c| {
            (
                c.id,
                Category {
                    id: c.id,
                    parent_id: c.parent_id,
                    code: c.code.clone(),
                    name_ru: c.name_ru.clone(),
                    name_en: c.name_en.clone(),
                    name_gr: c.name_gr.clone(),
                    depth: c.depth,
                    sort_order: c.sort_order,
                    description: c.description.clone(),
                    children: vec![],
                },
            )
        })
        .collect();

    // Sort ids by depth then sort_order
    let mut ids: Vec<i64> = map.keys().copied().collect();
    ids.sort_by_key(|id| {
        let c = &map[id];
        (c.depth, c.sort_order)
    });

    // Attach children bottom-up (skip root id 0)
    let leaf_ids: Vec<i64> = ids.iter().copied().filter(|&id| id != 0).rev().collect();
    for id in leaf_ids {
        let parent_id = map[&id].parent_id;
        if let Some(pid) = parent_id {
            if pid != 0 && map.contains_key(&pid) {
                let child = map.remove(&id).unwrap();
                if let Some(parent) = map.get_mut(&pid) {
                    parent.children.push(child);
                }
            }
        }
    }

    // Return all remaining top-level categories sorted
    let mut top: Vec<Category> = map.into_values().filter(|c| c.id != 0).collect();
    top.sort_by_key(|c| c.sort_order);
    top
}
