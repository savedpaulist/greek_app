use rusqlite::{Connection, Result};
use serde::Serialize;
use std::{env, fs, path::Path};

#[derive(Serialize)]
struct LemmaRaw {
    id: i64,
    greek: String,
    latin: Option<String>,
    english: Option<String>,
    russian: Option<String>,
    part_of_speech: Option<String>,
    category_id: Option<i64>,
    lesson_kozar: Option<i32>,
    lesson_mast: Option<i32>,
    sort_order: i32,
}

#[derive(Serialize)]
struct FormRaw {
    id: i64,
    lemma_id: i64,
    greek_form: String,
    transliteration: Option<String>,
    category_id: Option<i64>,
    pos: Option<String>,
    case_tag: Option<String>,
    number_tag: Option<String>,
    gender_tag: Option<String>,
    tense_tag: Option<String>,
    voice_tag: Option<String>,
    mood_tag: Option<String>,
    person_tag: Option<String>,
    degree_tag: Option<String>,
    decl_type: Option<String>,
    conj_type: Option<String>,
    adj_class: Option<String>,
    part_type: Option<String>,
    num_type: Option<String>,
    pron_type: Option<String>,
    lesson_kozar: Option<i32>,
    lesson_mast: Option<i32>,
    paradigm_row: Option<i32>,
    sort_order: i32,
    notes: Option<String>,
}

#[derive(Serialize)]
struct CategoryRaw {
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

#[derive(Serialize)]
struct TagRaw {
    id: i64,
    group_name: String,
    tag_value: String,
    label_ru: Option<String>,
    label_en: Option<String>,
    sort_order: i32,
}

#[derive(Serialize)]
struct DbDump {
    lemmas: Vec<LemmaRaw>,
    forms: Vec<FormRaw>,
    categories: Vec<CategoryRaw>,
    tags: Vec<TagRaw>,
}

fn main() -> Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let db_path = Path::new(&manifest_dir).join("assets").join("greek_paradigms.db");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("greek_data.json");

    println!("cargo:rerun-if-changed=assets/greek_paradigms.db");
    println!("cargo:rerun-if-changed=build.rs");

    if !db_path.exists() {
        // Create empty dump so the app can still compile without the DB
        let empty = DbDump { lemmas: vec![], forms: vec![], categories: vec![], tags: vec![] };
        fs::write(&out_path, serde_json::to_string(&empty).unwrap()).unwrap();
        return Ok(());
    }

    let conn = Connection::open(&db_path)?;

    // ── categories ───────────────────────────────────────────────────────────
    let mut stmt = conn.prepare(
        "SELECT id, parent_id, code, name_ru, name_en, name_gr, depth, sort_order, description
         FROM categories ORDER BY depth, sort_order",
    )?;
    let categories: Vec<CategoryRaw> = stmt
        .query_map([], |row| {
            Ok(CategoryRaw {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                code: row.get(2)?,
                name_ru: row.get(3)?,
                name_en: row.get(4)?,
                name_gr: row.get(5)?,
                depth: row.get(6)?,
                sort_order: row.get(7)?,
                description: row.get(8)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    // ── tags ─────────────────────────────────────────────────────────────────
    let mut stmt = conn.prepare(
        "SELECT id, group_name, tag_value, label_ru, label_en, sort_order FROM tags ORDER BY group_name, sort_order",
    )?;
    let tags: Vec<TagRaw> = stmt
        .query_map([], |row| {
            Ok(TagRaw {
                id: row.get(0)?,
                group_name: row.get(1)?,
                tag_value: row.get(2)?,
                label_ru: row.get(3)?,
                label_en: row.get(4)?,
                sort_order: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    // ── lemmas ───────────────────────────────────────────────────────────────
    let mut stmt = conn.prepare(
        "SELECT id, greek, latin, english, russian, part_of_speech, category_id,
                lesson_kozar, lesson_mast, sort_order
         FROM lemmas ORDER BY id",
    )?;
    let lemmas: Vec<LemmaRaw> = stmt
        .query_map([], |row| {
            Ok(LemmaRaw {
                id: row.get(0)?,
                greek: row.get(1)?,
                latin: row.get(2)?,
                english: row.get(3)?,
                russian: row.get(4)?,
                part_of_speech: row.get(5)?,
                category_id: row.get(6)?,
                lesson_kozar: row.get(7)?,
                lesson_mast: row.get(8)?,
                sort_order: row.get(9)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    // ── forms ────────────────────────────────────────────────────────────────
    let mut stmt = conn.prepare(
        "SELECT id, lemma_id, greek_form, transliteration, category_id, pos,
                case_tag, number_tag, gender_tag, tense_tag, voice_tag, mood_tag,
                person_tag, degree_tag, decl_type, conj_type, adj_class, part_type,
                num_type, pron_type, lesson_kozar, lesson_mast, paradigm_row,
                COALESCE(sort_order, 0), notes
         FROM forms ORDER BY id",
    )?;
    let forms: Vec<FormRaw> = stmt
        .query_map([], |row| {
            Ok(FormRaw {
                id: row.get(0)?,
                lemma_id: row.get(1)?,
                greek_form: row.get(2)?,
                transliteration: row.get(3)?,
                category_id: row.get(4)?,
                pos: row.get(5)?,
                case_tag: row.get(6)?,
                number_tag: row.get(7)?,
                gender_tag: row.get(8)?,
                tense_tag: row.get(9)?,
                voice_tag: row.get(10)?,
                mood_tag: row.get(11)?,
                person_tag: row.get(12)?,
                degree_tag: row.get(13)?,
                decl_type: row.get(14)?,
                conj_type: row.get(15)?,
                adj_class: row.get(16)?,
                part_type: row.get(17)?,
                num_type: row.get(18)?,
                pron_type: row.get(19)?,
                lesson_kozar: row.get(20)?,
                lesson_mast: row.get(21)?,
                paradigm_row: row.get(22)?,
                sort_order: row.get(23)?,
                notes: row.get(24)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let dump = DbDump { lemmas, forms, categories, tags };
    fs::write(&out_path, serde_json::to_string(&dump).unwrap()).expect("write greek_data.json");

    Ok(())
}
