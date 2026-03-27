<!-- # σφώδρα — План разработки
## Rust + Dioxus WASM · Приложение для отработки древнегреческих парадигм

---

## 0. Обзор базы данных

Перед архитектурой — краткий портрет данных, которые приложение обслуживает:

| Таблица       | Строк  | Назначение                                              |
|---------------|--------|---------------------------------------------------------|
| `forms`       | 5 926  | Главная таблица: одна строка = одна грамматическая форма |
| `lemmas`      | 231    | Словарные леммы (существительные, глаголы, прил. и т.д.) |
| `categories`  | 58     | Иерархическое дерево разделов (depth 0–3)               |
| `tags`        | 39     | Справочник тегов (падеж, число, род, время, залог…)      |
| `progress`    | —      | Прогресс SM-2 по каждой форме                           |
| `sessions`    | —      | История сессий с filter_json                            |

**Грамматические атрибуты** в `forms` (денормализованы для скорости):
`case_tag` · `number_tag` · `gender_tag` · `tense_tag` · `voice_tag` · `mood_tag`
`person_tag` · `degree_tag` · `decl_type` · `conj_type` · `adj_class`
`part_type` · `num_type` · `pron_type`

**Типы склонений** (`decl_type`):
`1a_eta` · `1b_alpha` · `1c_masc_es` · `2a_os` · `2b_on` · `2c_attic` · `2d_contract`
`3a_sonant` · `3b_labial` · `3c_velar` · `3d_dental` · `3e_nt` · `3f_sigma` · `3g_vowel_iu` · `3h_diphth` · `3i_irreg_r`

**Типы спряжений** (`conj_type`):
`thematic` · `thematic_cons` · `contract_ao` · `contract_eo` · `contract_oo` · `mi_verb`

---

## 1. Стек и зависимости

```toml
[dependencies]
# Ядро
dioxus          = { version = "0.6", features = ["web", "router"] }
dioxus-web      = "0.6"

# Хранилище
rusqlite        = { version = "0.31", features = ["bundled", "wasm32-wasi-vfs"] }
# Альтернатива: rusqlite не компилируется в WASM — используем собственный
# SQLite через sql.js или хранение данных как встроенный ресурс (см. §4)

serde           = { version = "1", features = ["derive"] }
serde_json      = "1"

# Утилиты
unicode-normalization = "0.1"   # NFC-нормализация греческого текста
wasm-bindgen    = "0.2"
web-sys         = { version = "0.3", features = ["Window", "Storage", "Navigator"] }
gloo-storage    = "0.3"         # localStorage / IndexedDB обёртка
rand            = { version = "0.8", features = ["small_rng"] }
```

> **SQLite в WASM:** `rusqlite` с feature `bundled` компилируется в wasm32-unknown-unknown
> через `wasm32-wasi-vfs`. Для максимальной совместимости с Safari/iOS — использовать
> `sql.js` (js-binding) через `wasm-bindgen` + сериализовать прогресс в `localStorage`.
> Рекомендуется гибридная схема: данные (forms/lemmas) зашиты в бинарник как `include_bytes!`,
> прогресс хранится в `localStorage` как JSON.

---

## 2. Структура проекта

```
sfodra/
├── Cargo.toml
├── Dioxus.toml
├── assets/
│   ├── db/
│   │   └── greek_paradigms.db          # Встроен через include_bytes!
│   ├── fonts/
│   │   ├── GFS_Didot/                  # OTF шрифты
│   │   ├── GFS_Neohellenic/
│   │   ├── Noto_Serif_Greek/
│   │   ├── Cardo/
│   │   └── Gentium_Plus/
│   └── styles/
│       └── themes.css                  # CSS-переменные для всех тем
│
└── src/
    ├── main.rs                         # Точка входа + app shell
    │
    ├── db/
    │   ├── mod.rs                      # pub use
    │   ├── loader.rs                   # Парсинг встроенного .db через sql.js/sqlite_vfs
    │   ├── queries.rs                  # Все SQL-запросы → возвращают модели
    │   └── filter.rs                   # FilterParams → WHERE clause builder
    │
    ├── models/
    │   ├── mod.rs
    │   ├── form.rs                     # Form, Lemma, Category
    │   ├── tags.rs                     # TagGroup, Tag, все enum-типы тегов
    │   ├── progress.rs                 # ProgressRecord, SM2State
    │   ├── session.rs                  # Session, SessionMode
    │   └── filter.rs                   # FilterParams (всё что строит WHERE)
    │
    ├── state/
    │   ├── mod.rs
    │   ├── app_state.rs                # AppState — главное глобальное состояние
    │   ├── session_state.rs            # SessionState — текущая тренировка
    │   └── settings.rs                 # Settings — тема, шрифт, dual, диакритика
    │
    ├── logic/
    │   ├── mod.rs
    │   ├── sm2.rs                      # SM-2 алгоритм интервального повторения
    │   ├── quiz.rs                     # Логика режимов тестирования
    │   ├── paradigm.rs                 # Построение/проверка таблицы парадигм
    │   └── diacritics.rs               # Нормализация/сравнение с диакритикой
    │
    ├── router.rs                       # Route enum + Dioxus Router
    │
    └── ui/
        ├── mod.rs
        ├── app.rs                      # Корневой компонент <App />
        │
        ├── layout/
        │   ├── shell.rs                # <Shell> — боковая панель + контент
        │   ├── topbar.rs               # <TopBar> — заголовок + кнопки
        │   └── sidebar.rs              # <Sidebar> — фильтры + настройки (slide-in)
        │
        ├── pages/
        │   ├── home.rs                 # Главная: выбор режима / быстрый старт
        │   ├── paradigm_view.rs        # Просмотр полных парадигм
        │   ├── quiz_flashcard.rs       # Режим 2: слово → форма
        │   ├── quiz_reverse.rs         # Режим 3: форма → слово
        │   ├── quiz_fill.rs            # Режим 4: ввод формы
        │   ├── quiz_build.rs           # Режим 5: собери парадигму
        │   ├── progress.rs             # Экран прогресса пользователя
        │   └── settings.rs             # Настройки
        │
        └── components/
            ├── filter_panel.rs         # Панель фильтров (dropdown/sidebar)
            ├── paradigm_table.rs       # Таблица парадигмы
            ├── flashcard.rs            # Карточка для режимов 2/3
            ├── fill_input.rs           # Поле ввода греческой формы
            ├── build_grid.rs           # Сетка для режима 5
            ├── progress_bar.rs         # Полоса прогресса
            ├── score_badge.rs          # Бейдж с % правильных
            ├── theme_picker.rs         # Выбор темы
            ├── font_picker.rs          # Выбор шрифта
            └── category_tree.rs        # Дерево категорий для фильтра
```

---

## 3. Модели данных (`src/models/`)

### 3.1 Теги — enum-типы

```rust
// src/models/tags.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Case { Nom, Gen, Dat, Acc, Voc }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Number { Sg, Du, Pl }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender { M, F, N }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tense { Pres, Imperf, Fut, Aor1, Aor2, AorPass, Perf, Pluperf, Futperf }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Voice { Act, Mid, Pass, MidPass }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mood { Ind, Subj, Opt, Imp, Inf, Part }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Person { P1, P2, P3 }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Degree { Pos, Comp, Superl }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeclType {
    Eta, Alpha, MascEs,
    Os, On, Attic, Contract2,
    Sonant, Labial, Velar, Dental, Nt, Sigma,
    VowelIU, Diphth, IrregR, Unknown3
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConjType { Thematic, ThematicCons, ContractAo, ContractEo, ContractOo, MiVerb }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartOfSpeech { Noun, Verb, Adj, Pronoun, Article, Num, Participle, Inf, Term, Letter }

// Каждый enum реализует Display (греч./рус./англ.), FromStr, итерацию всех вариантов
impl Case {
    pub fn all() -> &'static [Case] { &[Case::Nom, Case::Gen, Case::Dat, Case::Acc, Case::Voc] }
    pub fn label_ru(&self) -> &'static str { match self { Case::Nom => "Им.", ... } }
    pub fn label_en(&self) -> &'static str { match self { Case::Nom => "Nom.", ... } }
    pub fn from_db(s: &str) -> Option<Self> { match s { "nom" => Some(Self::Nom), ... } }
    pub fn to_db(&self) -> &'static str { match self { Self::Nom => "nom", ... } }
}
// аналогично для всех остальных enum
```

### 3.2 Основные модели

```rust
// src/models/form.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lemma {
    pub id: i64,
    pub greek: String,
    pub english: Option<String>,
    pub russian: Option<String>,
    pub pos: PartOfSpeech,
    pub category_id: Option<i64>,
    pub lesson_kozar: Option<i32>,
    pub lesson_mast: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: i64,
    pub lemma_id: i64,
    pub greek_form: String,
    pub transliteration: Option<String>,
    pub category_id: Option<i64>,
    pub pos: PartOfSpeech,
    // Грамматические атрибуты
    pub case_tag: Option<Case>,
    pub number_tag: Option<Number>,
    pub gender_tag: Option<Gender>,
    pub tense_tag: Option<Tense>,
    pub voice_tag: Option<Voice>,
    pub mood_tag: Option<Mood>,
    pub person_tag: Option<Person>,
    pub degree_tag: Option<Degree>,
    pub decl_type: Option<DeclType>,
    pub conj_type: Option<ConjType>,
    pub part_type: Option<String>,
    pub num_type: Option<String>,
    pub pron_type: Option<String>,
    pub lesson_kozar: Option<i32>,
    pub lesson_mast: Option<i32>,
    pub paradigm_row: Option<i32>,
    pub sort_order: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub code: String,
    pub name_ru: String,
    pub name_en: String,
    pub name_gr: Option<String>,
    pub depth: i32,
    pub sort_order: i32,
    pub description: Option<String>,
    pub children: Vec<Category>,  // заполняется при загрузке дерева
}
```

### 3.3 Прогресс и сессия

```rust
// src/models/progress.rs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgressRecord {
    pub form_id: i64,
    pub correct: u32,
    pub incorrect: u32,
    pub streak: u32,
    pub ease_factor: f32,   // SM-2, default 2.5
    pub interval_days: u32, // default 1
    pub next_review: i64,   // Unix timestamp
    pub last_seen: i64,
}

impl ProgressRecord {
    pub fn accuracy(&self) -> f32 {
        let total = self.correct + self.incorrect;
        if total == 0 { 0.0 } else { self.correct as f32 / total as f32 }
    }
}

// src/models/session.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionMode {
    ParadigmView,
    Flashcard,      // слово → форма
    Reverse,        // форма → слово
    FillIn,         // ввод формы
    BuildParadigm,  // расставить ячейки
    Spaced,         // интервальное повторение
}
```

---

## 4. Хранилище: данные + прогресс

### 4.1 Данные (read-only)

База данных встраивается в бинарник:

```rust
// src/db/loader.rs
static DB_BYTES: &[u8] = include_bytes!("../../assets/db/greek_paradigms.db");
```

В WASM используется `sql.js` через `wasm-bindgen`:
- При старте приложения `DB_BYTES` передаётся в `SQL.Database(new Uint8Array(bytes))`
- Все запросы — синхронные (in-memory база)
- Альтернатива: при первом запуске скопировать в `IndexedDB` через `idb-keyval`

### 4.2 Прогресс (read-write)

```rust
// src/state/app_state.rs
// Прогресс сериализуется как HashMap<i64, ProgressRecord> → JSON → localStorage
// Ключ: "sfodra_progress_v1"
// При каждом ответе: обновить запись → serialize → localStorage.setItem

pub struct AppState {
    pub progress: HashMap<i64, ProgressRecord>,
    pub settings: Settings,
    pub categories: Vec<Category>,  // дерево, загружается один раз
    pub sessions: Vec<Session>,
}
```

---

## 5. Фильтр (`FilterParams`)

```rust
// src/models/filter.rs

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FilterParams {
    // Категории
    pub category_ids: Vec<i64>,         // дерево — включает дочерние
    pub pos: Vec<PartOfSpeech>,

    // Именные
    pub cases: Vec<Case>,
    pub numbers: Vec<Number>,           // включать dual?
    pub genders: Vec<Gender>,
    pub decl_types: Vec<DeclType>,

    // Глагольные
    pub tenses: Vec<Tense>,
    pub voices: Vec<Voice>,
    pub moods: Vec<Mood>,
    pub persons: Vec<Person>,
    pub conj_types: Vec<ConjType>,

    // Прилагательные
    pub degrees: Vec<Degree>,

    // По урокам
    pub lesson_kozar_min: Option<i32>,
    pub lesson_kozar_max: Option<i32>,
    pub lesson_mast_min: Option<i32>,
    pub lesson_mast_max: Option<i32>,

    // Флаги
    pub include_dual: bool,             // по умолчанию false
    pub only_due: bool,                 // только требующие повторения (SM-2)
    pub only_weak: bool,                // streak < 3
    pub exclude_learned: bool,          // streak >= 5
}

impl FilterParams {
    /// Строит WHERE-клаузу для SQLite.
    /// Возвращает (sql_fragment, params)
    pub fn to_sql(&self) -> (String, Vec<SqlParam>) { ... }

    /// Проверяет, пустой ли фильтр (= "всё")
    pub fn is_empty(&self) -> bool { ... }

    /// Пресеты — готовые "темы для изучения"
    pub fn preset_kozar_1_5() -> Self { ... }
    pub fn preset_mast_1_10() -> Self { ... }
    pub fn preset_1st_decl() -> Self { ... }
    pub fn preset_thematic_present() -> Self { ... }
    // ... и т.д. для каждой категории
}
```

---

## 6. SM-2 алгоритм

```rust
// src/logic/sm2.rs

/// Качество ответа: 0–5
/// 0 = полный провал, 3 = правильно с трудом, 5 = мгновенно
pub type Quality = u8;

/// Применяет один шаг SM-2 к записи прогресса
pub fn sm2_update(rec: &mut ProgressRecord, quality: Quality) {
    assert!(quality <= 5);

    if quality >= 3 {
        rec.interval_days = match rec.streak {
            0 => 1,
            1 => 6,
            _ => (rec.interval_days as f32 * rec.ease_factor).round() as u32,
        };
        rec.streak += 1;
    } else {
        rec.streak = 0;
        rec.interval_days = 1;
    }

    rec.ease_factor = (rec.ease_factor
        + 0.1 - (5 - quality) as f32 * (0.08 + (5 - quality) as f32 * 0.02))
        .max(1.3);

    let now = js_sys::Date::now() as i64 / 1000;
    rec.next_review = now + rec.interval_days as i64 * 86400;
    rec.last_seen = now;
}

/// Маппинг режимов → качество
/// Fill-in правильно без ошибок → quality 5
/// Flashcard с подсказкой → quality 3
/// Неправильно → quality 1
pub fn quality_from_answer(correct: bool, used_hint: bool) -> Quality {
    match (correct, used_hint) {
        (true, false) => 5,
        (true, true)  => 3,
        (false, _)    => 1,
    }
}
```

---

## 7. Режимы работы

### 7.1 Режим 1 — Просмотр парадигм (`paradigm_view`)

- Выбрать лемму или категорию
- Отображать таблицу: строки = `paradigm_row`, столбцы = case×number или person×number
- Переключатель: "показать все формы / только выбранные по фильтру"
- Выделение цветом форм по прогрессу (зелёный = выучено, красный = слабо)
- Опция: показывать/скрывать перевод, транслитерацию

```rust
// src/logic/paradigm.rs

pub struct ParadigmTable {
    pub lemma: Lemma,
    pub rows: Vec<ParadigmRow>,     // строка = набор ячеек
    pub col_headers: Vec<String>,   // заголовки колонок
    pub row_headers: Vec<String>,   // заголовки строк
}

pub fn build_noun_paradigm(forms: &[Form], include_dual: bool) -> ParadigmTable { ... }
pub fn build_verb_paradigm(forms: &[Form], tenses: &[Tense]) -> ParadigmTable { ... }
pub fn build_adj_paradigm(forms: &[Form]) -> ParadigmTable { ... }
```

### 7.2 Режим 2 — Flashcard: слово → форма

- Показать: лемму + грамматическое описание (падеж, число, …)
- Пользователь называет форму (4 варианта для клика ИЛИ "раскрыть ответ")
- Ответ с self-report: "знал" / "с трудом" / "не знал"

### 7.3 Режим 3 — Flashcard reverse: форма → слово

- Показать: форму + грамматические теги
- Пользователь называет словарную форму (4 варианта)

### 7.4 Режим 4 — Fill-in: ввести форму

```rust
// src/logic/diacritics.rs

/// Сравнивает ответ с эталоном
/// ignore_diacritics: убирает акценты, придыхания, ι-subscript
pub fn compare_greek(answer: &str, expected: &str, ignore_diacritics: bool) -> bool {
    let a = normalize(answer, ignore_diacritics);
    let e = normalize(expected, ignore_diacritics);
    a == e
}

fn normalize(s: &str, strip: bool) -> String {
    use unicode_normalization::UnicodeNormalization;
    let nfd: String = s.nfd().collect();
    if strip {
        // убрать combining diacritical marks (U+0300–U+036F) + combining ι subscript
        nfd.chars().filter(|c| !is_greek_combining(*c)).collect()
    } else {
        nfd.nfc().collect()
    }
}
```

- Показать: словарную форму + грамматическое описание
- Пользователь вводит форму в текстовое поле
- Если нет клавиатуры с диакритикой → сравнение без диакритики
- Подсветка diff: зелёный/красный по символам

### 7.5 Режим 5 — Собери парадигму

```rust
// src/logic/quiz.rs

pub struct BuildParadigmQuiz {
    pub lemma: Lemma,
    pub grid: Vec<GridCell>,       // пустые ячейки
    pub draggable: Vec<Form>,      // перетаскиваемые формы (+ дистракторы)
}

pub struct GridCell {
    pub row_label: String,
    pub col_label: String,
    pub expected_form_id: i64,
    pub placed_form: Option<Form>,
}
```

- Показать пустую таблицу + набор форм снизу
- Drag-and-drop (или tap-to-place на мобильном)
- Несколько форм = дистракторы из похожих лемм

---

## 8. Состояние приложения

```rust
// src/state/app_state.rs
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub progress: Signal<HashMap<i64, ProgressRecord>>,
    pub settings: Signal<Settings>,
    pub filter: Signal<FilterParams>,
    pub categories: Signal<Vec<Category>>,   // дерево, read-only
}

impl AppState {
    pub fn load_from_storage() -> Self { ... }
    pub fn save_progress(&self) { ... }  // → localStorage
    pub fn reset_progress(&mut self) { ... }
}

// src/state/settings.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub font: GreekFont,
    pub include_dual: bool,
    pub ignore_diacritics: bool,
    pub has_diacritic_keyboard: Option<bool>,  // None = не спрашивали
    pub language: UiLanguage,   // Ru / En
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    GruvboxLight, GruvboxDark,
    DraculaLight, DraculaDark,
    GreenMossLight, GreenMossDark,
    PinkPastelLight, PinkPastelDark,
    SeriousBlueLight, SeriousBlueDark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GreekFont { GfsDidot, GfsNeohellenic, NotoSerifGreek, Cardo, GentiumPlus }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UiLanguage { Ru, En }
```

---

## 9. Роутинг

```rust
// src/router.rs
use dioxus_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/view/:lemma_id")]
    ParadigmView { lemma_id: i64 },

    #[route("/quiz/flashcard")]
    QuizFlashcard {},

    #[route("/quiz/reverse")]
    QuizReverse {},

    #[route("/quiz/fill")]
    QuizFill {},

    #[route("/quiz/build")]
    QuizBuild {},

    #[route("/progress")]
    Progress {},

    #[route("/settings")]
    Settings {},
}
```

---

## 10. UI — компоненты

### 10.1 Shell и навигация

```
┌─────────────────────────────────────────────────┐
│  TopBar: [☰ σφώδρα]          [🎯 режим] [⚙️]   │
├─────────────────────────────────────────────────┤
│                                                 │
│              <Page Content>                     │
│                                                 │
│ (iPhone SE: 375×667 — всё помещается)           │
└─────────────────────────────────────────────────┘

Sidebar (slide-in справа):
┌─────────────────────────┐
│  🔍 Фильтры             │
│  ───────────────────    │
│  Часть речи   [▼]       │
│  Категория    [▼]       │
│  Падеж        [▼]       │
│  Число        [▼]       │
│  ...                    │
│  ───────────────────    │
│  📚 Темы для изучения   │
│  > 1-е склонение        │
│  > Тематические глаголы │
│  > Козарж. 1–10         │
│  ...                    │
└─────────────────────────┘
```

### 10.2 Flashcard

```
┌──────────────────────────────┐
│  [← ] Flashcard   12/48  [✕]│
│  ────────────────────────── │
│                              │
│        καλός                 │
│     "прекрасный"             │
│                              │
│  Дательный · Мн.ч. · Ж.р.  │
│                              │
│ ┌──────────┐ ┌──────────┐   │
│ │ καλαῖς   │ │ καλοῖς   │   │
│ ├──────────┤ ├──────────┤   │
│ │ καλαῖν   │ │ καλαῖς   │   │
│ └──────────┘ └──────────┘   │
│                              │
│         [Показать]           │
└──────────────────────────────┘
```

### 10.3 Fill-in

```
┌──────────────────────────────┐
│  Fill in   ——————————●  8/20 │
│  ────────────────────────── │
│                              │
│        λύω                   │
│  Аорист I · Акт · Инд        │
│  2-е л. · Ед.ч.              │
│                              │
│  ┌────────────────────────┐  │
│  │  ἔλυσας_               │  │
│  └────────────────────────┘  │
│                              │
│  [← Удалить]    [Ввести →]   │
│                              │
│  ⌨️ без диакритики [вкл.]    │
└──────────────────────────────┘
```

### 10.4 Build Paradigm

```
┌──────────────────────────────┐
│  Собери парадигму   καλός    │
│  ────────────────────────── │
│       Ед.   Дв.  Мн.        │
│  Им. [ ? ] [  ] [ ? ]       │
│  Род [ ? ] [  ] [ ? ]       │
│  Дат [ ? ] [  ] [✓]         │
│  ────────────────────────── │
│  καλός  καλοῦ  καλῷ         │
│  καλόν  καλαί  καλῶν        │
│                              │
│  (перетащить в ячейку)       │
└──────────────────────────────┘
```

---

## 11. Темы (CSS-переменные)

```css
/* assets/styles/themes.css */

/* ── GruvBox Light ──────────────────────────── */
[data-theme="gruvbox-light"] {
  --bg:          #fbf1c7;
  --bg-soft:     #f2e5bc;
  --surface:     #ebdbb2;
  --border:      #d5c4a1;
  --text:        #3c3836;
  --text-muted:  #928374;
  --accent:      #b57614;
  --accent-soft: #d79921;
  --green:       #79740e;
  --red:         #9d0006;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── GruvBox Dark ───────────────────────────── */
[data-theme="gruvbox-dark"] {
  --bg:          #282828;
  --bg-soft:     #32302f;
  --surface:     #3c3836;
  --border:      #504945;
  --text:        #ebdbb2;
  --text-muted:  #a89984;
  --accent:      #fabd2f;
  --accent-soft: #d79921;
  --green:       #b8bb26;
  --red:         #fb4934;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── Dracula Light ──────────────────────────── */
[data-theme="dracula-light"] {
  --bg:          #f8f8f2;
  --bg-soft:     #f0f0e8;
  --surface:     #e8e8e0;
  --border:      #d0d0c8;
  --text:        #282a36;
  --text-muted:  #6272a4;
  --accent:      #bd93f9;
  --accent-soft: #caa9fa;
  --green:       #50fa7b;
  --red:         #ff5555;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── Dracula Dark ───────────────────────────── */
[data-theme="dracula-dark"] {
  --bg:          #282a36;
  --bg-soft:     #21222c;
  --surface:     #343746;
  --border:      #44475a;
  --text:        #f8f8f2;
  --text-muted:  #6272a4;
  --accent:      #bd93f9;
  --accent-soft: #ff79c6;
  --green:       #50fa7b;
  --red:         #ff5555;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── GreenMoss Light ────────────────────────── */
[data-theme="greenmoss-light"] {
  --bg:          #f5f7f0;
  --bg-soft:     #eaefe0;
  --surface:     #dde5cc;
  --border:      #b8c9a0;
  --text:        #2d3820;
  --text-muted:  #6b7a58;
  --accent:      #5a7a3a;
  --accent-soft: #7a9a50;
  --green:       #3a6020;
  --red:         #8a2020;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── GreenMoss Dark ─────────────────────────── */
[data-theme="greenmoss-dark"] {
  --bg:          #1e2418;
  --bg-soft:     #252c1e;
  --surface:     #2e3828;
  --border:      #3e4e30;
  --text:        #d8e8c0;
  --text-muted:  #8a9a70;
  --accent:      #90c060;
  --accent-soft: #70a040;
  --green:       #a0d870;
  --red:         #e06060;
  --radius:      12px;
  --radius-sm:   8px;
}

/* ── PinkPastel Light ───────────────────────── */
[data-theme="pinkpastel-light"] {
  --bg:          #fff5f8;
  --bg-soft:     #ffe8f0;
  --surface:     #ffd8e8;
  --border:      #f0b8cc;
  --text:        #3a1828;
  --text-muted:  #987088;
  --accent:      #c0507a;
  --accent-soft: #e080a0;
  --green:       #607060;
  --red:         #d04060;
  --radius:      14px;
  --radius-sm:   10px;
}

/* ── PinkPastel Dark ────────────────────────── */
[data-theme="pinkpastel-dark"] {
  --bg:          #281822;
  --bg-soft:     #32202a;
  --surface:     #3e2832;
  --border:      #582840;
  --text:        #f0d8e8;
  --text-muted:  #b088a0;
  --accent:      #e080a8;
  --accent-soft: #c060a0;
  --green:       #90c090;
  --red:         #e06080;
  --radius:      14px;
  --radius-sm:   10px;
}

/* ── SeriousBlue Light ──────────────────────── */
[data-theme="seriousblue-light"] {
  --bg:          #f0f4f8;
  --bg-soft:     #e4ecf4;
  --surface:     #d8e4f0;
  --border:      #b0c8e0;
  --text:        #1a2840;
  --text-muted:  #607090;
  --accent:      #2060a8;
  --accent-soft: #4080c8;
  --green:       #205840;
  --red:         #802020;
  --radius:      10px;
  --radius-sm:   6px;
}

/* ── SeriousBlue Dark ───────────────────────── */
[data-theme="seriousblue-dark"] {
  --bg:          #0e1a28;
  --bg-soft:     #162030;
  --surface:     #1e2c40;
  --border:      #2a3e58;
  --text:        #d0e4f8;
  --text-muted:  #708aaa;
  --accent:      #4090e0;
  --accent-soft: #60a8f0;
  --green:       #40b870;
  --red:         #e05050;
  --radius:      10px;
  --radius-sm:   6px;
}
```

---

## 12. Шрифты для древнегреческого

| ID              | Название          | Характер                          | Источник          |
|-----------------|-------------------|-----------------------------------|-------------------|
| `GfsDidot`      | GFS Didot         | Классический, академический       | greekfontsociety.org |
| `GfsNeohellenic`| GFS Neohellenic   | Современный, читаемый             | greekfontsociety.org |
| `NotoSerifGreek`| Noto Serif Greek  | Системный, нейтральный            | Google Fonts      |
| `Cardo`         | Cardo             | Для классиков, с диакритикой      | SIL Open Font     |
| `GentiumPlus`   | Gentium Plus      | Отличная поддержка диакритики     | SIL Open Font     |

Все шрифты — SIL Open Font License, свободное использование в WASM.

---

## 13. Готовые темы для изучения (пресеты фильтров)

```rust
// В src/models/filter.rs — ассоциированные функции FilterParams:

// Имена      → код → описание фильтра
pub fn preset(id: &str) -> Option<FilterParams> {
    match id {
        // СУЩЕСТВИТЕЛЬНЫЕ
        "noun_1st_all"      => /* decl: 1a/1b/1c, pos: noun */
        "noun_1st_eta"      => /* decl_type: 1a_eta */
        "noun_1st_alpha"    => /* decl_type: 1b_alpha */
        "noun_2nd_all"      => /* decl: 2a/2b/2c/2d */
        "noun_3rd_all"      => /* decl: 3a..3i */
        "noun_3rd_dental"   => /* decl_type: 3d_dental */
        "noun_3rd_sigma"    => /* decl_type: 3f_sigma */

        // ПРИЛАГАТЕЛЬНЫЕ
        "adj_1_2_3end"      => /* adj_class: 1_2_3end */
        "adj_3rd_2end"      => /* adj_class: 3rd_2end */
        "adj_mixed"         => /* adj_class: mixed */
        "adj_comparison"    => /* degree: comp, superl */

        // ГЛАГОЛЫ (настоящее время)
        "verb_pres_ind"     => /* tense: pres, mood: ind */
        "verb_pres_all_moods" => /* tense: pres, mood: ind/subj/opt/imp */
        "verb_contract_ao"  => /* conj_type: contract_ao */
        "verb_contract_eo"  => /* conj_type: contract_eo */
        "verb_mi_all"       => /* conj_type: mi_verb */

        // ГЛАГОЛЫ (исторические времена)
        "verb_imperf"       => /* tense: imperf */
        "verb_aor1_act"     => /* tense: aor1, voice: act */
        "verb_aor1_pass"    => /* tense: aor_pass */
        "verb_perf_act"     => /* tense: perf, voice: act */

        // МЕСТОИМЕНИЯ
        "pron_personal"     => /* pron_type: personal */
        "pron_demonstr"     => /* pron_type: demonstr */
        "pron_all"          => /* pos: pronoun */

        // ПО УЧЕБНИКАМ
        "kozar_01_05"       => /* lesson_kozar: 1–5 */
        "kozar_06_10"       => /* lesson_kozar: 6–10 */
        "kozar_11_20"       => /* lesson_kozar: 11–20 */
        "mast_01_10"        => /* lesson_mast: 1–10 */
        "mast_11_20"        => /* lesson_mast: 11–20 */

        // СМЕШАННЫЕ
        "article"           => /* pos: article */
        "participles_all"   => /* pos: participle */
        "numbers_all"       => /* pos: num */
        "spaced_due"        => /* only_due: true */
        "weakest_50"        => /* only_weak: true, limit 50 */
        _ => None,
    }
}
```

---

## 14. Прогресс пользователя — экран статистики

```
┌──────────────────────────────┐
│  Прогресс                    │
│  ────────────────────────── │
│  Всего форм:      5 926      │
│  Встречено:       482 (8%)   │
│  Выучено (≥5):    134        │
│  ────────────────────────── │
│  По категориям               │
│  Существительные ████░ 42%  │
│  Глаголы         ██░░░ 21%  │
│  Прилагательные  ███░░ 38%  │
│  ────────────────────────── │
│  Сессий:  18                 │
│  Точность: 73%               │
│                              │
│  [Сбросить прогресс]         │
└──────────────────────────────┘
```

---

## 15. Первый запуск — онбординг

При первом открытии приложение задаёт **один** вопрос:

```
У вас есть греческая клавиатура
с диакритическими знаками?

  [Да]  [Нет]  [Спрошу позже]
```

Если **Да** → второй вопрос:
```
Учитывать диакритику при проверке
вводимых форм?

  [Да, строго]  [Нет, игнорировать]
```

Настройка сохраняется в `Settings.ignore_diacritics` и `Settings.has_diacritic_keyboard`.
Всегда доступна в Settings → «Учитывать диакритику в тесте».

---

## 16. Адаптивность (iPhone SE и выше)

- **Базовый viewport:** 375×667 (iPhone SE 2020)
- Все кнопки: минимальный touch target 44×44px
- Шрифт основного контента: 16px (греческий) / 14px (подписи)
- Sidebar: 85vw на мобильном, 320px на десктопе
- Таблица парадигмы: горизонтальный scroll на малом экране
- Карточка flashcard: занимает 70% высоты экрана
- Нижняя навигация скрыта — всё через TopBar + Sidebar

```css
/* Базовые ограничения */
.app-shell {
    max-width: 480px;   /* не растягивать на планшете */
    margin: 0 auto;
    min-height: 100dvh;
}

.paradigm-table-wrapper {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
}

.btn-primary {
    min-height: 44px;
    border-radius: var(--radius-sm);
    font-size: 16px;
}
```

---

## 17. Порядок разработки (этапы)

### Этап 1 — Ядро (2–3 нед.)
1. Настройка проекта Dioxus + Trunk + wasm-pack
2. Встраивание `.db` через `include_bytes!` + sql.js bindings
3. Реализация всех моделей и enum-типов
4. `FilterParams::to_sql()` + тесты
5. `AppState` + сериализация прогресса в `localStorage`
6. Базовая маршрутизация

### Этап 2 — Режим просмотра (1 нед.)
7. `ParadigmTable` builder для noun/verb/adj
8. Компонент `<ParadigmTableView>`
9. Компонент `<CategoryTree>` + фильтр-панель

### Этап 3 — Тестирование (2–3 нед.)
10. Режим Flashcard (2) + Reverse (3)
11. SM-2 + прогресс
12. Режим Fill-in (4) + `compare_greek()`
13. Режим Build Paradigm (5)

### Этап 4 — UI-полировка (1–2 нед.)
14. Все 10 тем + CSS переменные
15. 5 шрифтов
16. Адаптивность под iPhone SE
17. Анимации (fade, slide-in) — только CSS transitions

### Этап 5 — Финал
18. Пресеты фильтров
19. Экран прогресса
20. PWA манифест + service worker (для offline на iOS)
21. Онбординг-экран
22. Тестирование на реальном iPhone SE

---

## 18. Ключевые принципы кода

```rust
// ✅ Хорошо: trait для отображения тегов, не дублировать match везде
pub trait TagDisplay {
    fn label_ru(&self) -> &'static str;
    fn label_en(&self) -> &'static str;
    fn to_db(&self) -> &'static str;
    fn all_variants() -> &'static [Self] where Self: Sized;
}

// ✅ Хорошо: builder-паттерн для запросов
FilterParams::default()
    .with_pos(PartOfSpeech::Verb)
    .with_tense(Tense::Pres)
    .with_mood(Mood::Ind)
    .only_due()

// ✅ Хорошо: компоненты без лишних пропсов через Signal/Context
// Не пробрасывать AppState через props — использовать use_context()

// ✅ Хорошо: все строки локализованы через единую функцию
fn t(key: &str, lang: UiLanguage) -> &'static str

// ❌ Плохо: дублировать SQL в разных местах
// ❌ Плохо: хранить прогресс в Component state
// ❌ Плохо: render-блокирующие операции в event handlers
```

---

## 19. PWA — работа офлайн на iPhone

```json
// Dioxus.toml → assets/manifest.json
{
  "name": "σφώδρα",
  "short_name": "σφώδρα",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#fbf1c7",
  "theme_color": "#b57614",
  "icons": [
    { "src": "/icons/192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/icons/512.png", "sizes": "512x512", "type": "image/png" }
  ]
}
```

Service Worker (Workbox) кэширует:
- WASM модуль (~2MB)
- `.db` файл (~1MB) — кэшируется при первом запуске
- CSS + шрифты

На iOS: пользователь добавляет в «На экране "Домой"» → полноэкранный режим,
работает без интернета.

---

## 20. Финальная структура команд

```bash
# Разработка
trunk serve --open

# Сборка для деплоя
trunk build --release

# Тестирование логики (без WASM)
cargo test --lib

# Проверка типов
cargo check --target wasm32-unknown-unknown
```

---

*Приложение σφώδρα — «усиленно», «стремительно» (др.-греч.) — название отражает характер работы с парадигмами.* -->
