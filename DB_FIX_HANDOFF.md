# Greek Paradigms DB — Handoff Document
*Дата: 2026-04-16*

---

## Контекст проекта

**Приложение**: Rust/Dioxus WASM-приложение для изучения древнегреческих парадигм (Attic Greek).  
**База данных**: `assets/greek_paradigms.db` (SQLite)  
**Бэкап**: `assets/greek_paradigms.db.bak3` (создан до исправлений)

Схема: таблица `forms` содержит поля:
```
id, lemma_id, greek_form, pos, tense_tag, voice_tag, mood_tag, person_tag, number_tag, case_tag, gender_tag, dialect_tag
```

Таблица `lemmas`: `id, greek, pos, russian, english, ...`

---

## Что было сделано (ЗАВЕРШЕНО)

### ✅ Кат. A — Плюсквамперфект под тегом `perf` → исправить на `pluperf`

Плюсквамперфект = аугмент ε + удвоение + окончания (-κειν/-κεις/-κει/-κεσαν для акт.; -μην/-νto для пасс.).

**Исправлено для глаголов:**

| Глагол | lemma_id | SQL-фильтр |
|--------|----------|------------|
| φιλέω | 202 | `id IN (2768..2783)` — актив: ἐπεφιλήκειν..ἐπεφιλήκεσαν; пассив: ἐπεφιλήμην..ἐπεφίληντο |
| λύω | 207 | формы с `ἐλελύ%` и `ἐλελ%` |
| λείπω | 208 | формы с `ἐλελειπ%` / `ἐλελίπ%` |
| τῑμάω | 209 | формы с `ἐτετιμ%` |
| δίδωμι | 204 | формы с `ἐδεδώ%` / `ἐδεδίδ%` |
| γιγνώσκω | 214 | формы с `ἐγεγνώ%` (под `perf|act|ind`) |
| φαίνω | 216 | актив: ἐπεφήν-; пассив: ἐπεφάν- |
| βαίνω | 210 | формы с `ἐβεβήκ%` |
| ἵστημι | 206 | формы с `ἑιστήκ%` / `εἱστήκ%` |

Все изменения: `UPDATE forms SET tense_tag='pluperf' WHERE ...`

---

### ✅ Кат. B — Futurum perfecti под тегом `fut|mid` → исправить на `futperf|pass`

Futperf пасс. = πεφιλήσομαι, λελείψομαι (редупликация + σομαι).

**Исправлено:**
- φιλέω (202): id 2784–2799 → `tense_tag='futperf', voice_tag='pass'`
- λείπω (208): формы λελείψομαι... → `futperf|pass`
- ἵστημι (206): ἑστήξομαι — УЖЕ был `futperf|mid` в БД, не трогали

---

### ✅ Кат. C — φαίνω fut|mid: ind↔opt перепутаны

Формы φανοῖμι/φανοῖς/φανοῖ (это ОПТАТИВ) были помечены как `ind`.  
Формы φανοῦμαι/φανῇ/φανεῖται (это ИНДИКАТИВ) были помечены как `opt`.

**Исправлено** 3-шаговым свопом через временный тег `_tmp_ind`:
```sql
UPDATE forms SET mood_tag='_tmp_ind' WHERE lemma_id=216 AND tense_tag='fut' AND voice_tag='mid' AND mood_tag='ind';
UPDATE forms SET mood_tag='ind' WHERE lemma_id=216 AND tense_tag='fut' AND voice_tag='mid' AND mood_tag='opt';
UPDATE forms SET mood_tag='opt' WHERE lemma_id=216 AND tense_tag='fut' AND voice_tag='mid' AND mood_tag='_tmp_ind';
```

---

### ✅ Кат. D — Имперфект актив под тегом `mid` → исправить на `act`

**Исправлено:**
- φιλέω (202): `UPDATE forms SET voice_tag='act' WHERE lemma_id=202 AND tense_tag='imperf' AND voice_tag='mid';` — 8 строк (ἐφίλουν, ἐφίλεις, ἐφίλει и т.д.)
- φαίνω (216): `UPDATE forms SET voice_tag='act' WHERE lemma_id=216 AND tense_tag='imperf' AND voice_tag='mid';` — 8 строк (ἔφαινον, ἔφαινες, ἔφαινε(ν) и т.д.)

---

### ✅ Кат. G — γιγνώσκω / δίδωμι: активный имперфект под `imperf|mid_pass` → `imperf|act`

**Активные формы** (отличаются от средне-пассивных по окончаниям: -ον/-ες/-ε(ν) vs -όμην/-ου/-ετο):

- γιγνώσκω (214): `UPDATE forms SET voice_tag='act' WHERE id IN (4901,4902,4903,4904,4905,4906,4907,4908);`
  - ἐγίγνωσκον (1sg), ἐγίγνωσκες (2sg), ἐγίγνωσκε(ν) (3sg), ἐγιγνώσκετον (du), ἐγιγνωσκέτην (2du), ἐγιγνώσκομεν (1pl), ἐγιγνώσκετε (2pl), ἐγίγνωσκον (3pl)

- δίδωμι (204): `UPDATE forms SET voice_tag='act' WHERE id IN (3127,3128,3129,3130,3131,3132,3133,3134);`
  - ἐδίδουν (1sg), ἐδίδους (2sg), ἐδίδου (3sg), ἐδίδοτον (du), ἐδιδότην (2du), ἐδίδομεν (1pl), ἐδίδοτε (2pl), ἐδίδοσαν (3pl)

---

### ✅ Кат. H — Битые данные

- Form id=2749 у φιλέω: `πεφιλημένοςπεφιλημένω` (склейка двух форм)
- `UPDATE forms SET greek_form='πεφιλημένος' WHERE id=2749;`

---

## Что ОСТАЛОСЬ сделать (НЕЗАВЕРШЕНО)

### ❌ Кат. E — βαίνω: смешение времён под `imperf` (lemma_id=210)

**Проблема**: В слотах `imperf|act|ind` сидят как минимум 4 разных времени:
- impf ἔβαινον (настоящий имперфект)
- fut βήσω (будущее)
- aor.II ἔβην (2-й аорист активный)
- aor.I ἔβησα (1-й аорист)

**Требует**: ручную разметку каждой формы по признакам:
- ἔβαινον → `imperf|act|ind` (сохранить)
- ἔβην, ἔβης, ἔβη, ἔβητον, ἐβήτην, ἔβημεν, ἔβητε, ἔβησαν → `aor2|act|ind`
- ἔβησα, ἔβησας, ἔβησε... → `aor1|act|ind`
- βήσομαι → `fut|mid|ind` (фактически это деп. форма)

**Как различить**:
1. Формы с -αιν- → impf act
2. Формы ἔβη-, ἔβη- (без σ) → aor2 act
3. Формы ἔβησ- (с σ) → aor1 act
4. Формы βήσ- (без аугмента, с σ) → fut

**SQL для проверки состояния**:
```sql
SELECT id, greek_form, tense_tag, voice_tag, mood_tag, person_tag, number_tag
FROM forms WHERE lemma_id=210
ORDER BY tense_tag, mood_tag, person_tag, number_tag;
```

---

### ❌ Кат. F — ἵστημι: аористные формы под `fut|mid` (lemma_id=206)

**Проблема**: Под `fut|mid` сидят три разных категории:
1. Истинный fut mid (στήσομαι и т.д.) — ОСТАВИТЬ как `fut|mid`
2. Аорист I средн. залог (ἐστησάμην и т.д.) — исправить на `aor1|mid`
3. Аорист II средн./акт. залог (ἐστάμην, στῆθι и т.д.) — исправить на `aor2|mid` или `aor2|act`

**Детали по ID-диапазонам** (из анализа):

| ID range | Формы | Нужный тег |
|----------|-------|-----------|
| 3711–3718 | στήσομαι, στήσῃ,, στήσεται, ... | **ОСТАВИТЬ** fut\|mid\|ind |
| 3719–3726 | στησοίμην, στήσοιο, ... | **ОСТАВИТЬ** fut\|mid\|opt |
| 3773–3780 | ἐστησάμην, ἐστήσω, ἐστήσατο, ... | → `aor1\|mid\|ind` |
| 3781–3788 | στήσωμαι, στήσῃ, στήσηται, ... | → `aor1\|mid\|subj` |
| 3789–3796 | στησαίμην, στήσαιο, στήσαιτο, ... | → `aor1\|mid\|opt` |
| 3797–3802 | στῆσαι, στησάσθω, στήσασθον, ... | → `aor1\|mid\|imp` |
| 3854–3859 | στῆθι, στήτω, στῆτον, στήτων, στῆτε, στάντων | → `aor2\|act\|imp` |
| 3860–3867 | ἐστάμην, ἔστω, ἔστατο, ... | → `aor2\|mid\|ind` |
| 3868–3875 | στῶμαι, στῇ, στῆται, ... | → `aor2\|mid\|subj` |
| 3876–3883 | σταίμην, σταῖο, σταῖτο, ... | → `aor2\|mid\|opt` |
| 3884–3889 | στῶ, στάσθω, στάσθον, ... | → `aor2\|mid\|imp` |

**SQL для исправления:**
```sql
-- Aorist I middle: all moods
UPDATE forms SET tense_tag='aor1'
WHERE id BETWEEN 3773 AND 3802 AND lemma_id=206;

-- Aorist II active imperative
UPDATE forms SET tense_tag='aor2', voice_tag='act'
WHERE id BETWEEN 3854 AND 3859 AND lemma_id=206;

-- Aorist II middle: indicative, subjunctive, optative, imperative
UPDATE forms SET tense_tag='aor2'
WHERE id BETWEEN 3860 AND 3889 AND lemma_id=206;
```

**Проверить перед запуском** (убедиться что диапазоны не сдвинулись):
```sql
SELECT id, greek_form, tense_tag, voice_tag, mood_tag
FROM forms WHERE lemma_id=206 AND id BETWEEN 3773 AND 3889
ORDER BY id;
```

---

### ❌ Кат. I — ἵημι: нестандартные формы (lemma_id=???)

**Задача**: Проверить все формы ἵημι под `fut|mid` — это, вероятно, тоже аористные формы (εἵμην, εἷσο, εἷτο и т.д.).

**SQL для анализа**:
```sql
SELECT id FROM lemmas WHERE greek='ἵημι';

SELECT id, greek_form, tense_tag, voice_tag, mood_tag, person_tag, number_tag
FROM forms WHERE lemma_id=??? AND tense_tag='fut'
ORDER BY voice_tag, mood_tag, person_tag, number_tag;
```

Нужна ручная проверка по грамматике Смита (Smyth).

---

### ❌ Добавить метку futperf в `tense_label()` в Rust-коде

Файл: `src/logic/paradigm.rs`, функция `tense_label()` (около строки 293).

Сейчас `futperf` показывается как сырая строка. Добавить:
```rust
"futperf" => "Fut.Pf",  // En
"futperf" => "Буд.перф.", // Ru
```

---

### ❌ Кат. E/F/I: Финальная проверка дублей

После всех исправлений запустить:
```sql
SELECT lemma_id, tense_tag, voice_tag, mood_tag, person_tag, number_tag, COUNT(*) as cnt
FROM forms
WHERE lemma_id IN (202,204,206,207,208,209,210,214,216)
GROUP BY lemma_id, tense_tag, voice_tag, mood_tag, person_tag, number_tag
HAVING cnt > 1
ORDER BY lemma_id, tense_tag;
```

Дубли — это баг: в квизе `.find()` берёт первую форму как правильный ответ, но вторая дублирующая форма появляется в списке вариантов и сбивает пользователя.

---

## Оставшиеся глаголы для проверки (не трогались)

Следующие 12 глаголов **не проверялись** и могут содержать аналогичные ошибки:

| Глагол | lemma_id | Вероятные ошибки |
|--------|----------|-----------------|
| βαίνω | 210 | Кат. E (КРИТИЧНО) |
| δέδοικα | ??? | Нет данных |
| δείκνῡμι | ??? | Нет данных |
| δηλόω | ??? | Плюскв. под `perf`? |
| δύναμαι | ??? | Нет данных |
| εἰμί | ??? | Нет данных |
| εἶμι | ??? | Нет данных |
| κεῖμαι | ??? | Нет данных |
| οἴομαι | ??? | Нет данных |
| οἶδα | ??? | Нет данных |
| τίθημι | ??? | Нет данных |
| φημί | ??? | Нет данных |

**Получить все lemma_id одним запросом:**
```sql
SELECT id, greek FROM lemmas WHERE pos='verb' ORDER BY greek;
```

---

## Справочная грамматика

**База-референс**: `assets/grammar-smyth-master/xhtml/greek_morphology.sqlite3`

Таблица `inflections`, `lemmas`. Глагол-образец для -εω: ποιέω (lemma_id=106). Содержит только Pres и Imperf.

**Распознавание времён по форме** (Active Indicative):
| Время | Признак |
|-------|---------|
| Pres | нет аугмента, окончания -ω/-εις/-ει |
| Imperf | аугмент ε-, тематические окончания -ον/-ες/-ε(ν) |
| Fut | σ-суффикс, нет аугмента |
| Aor I | аугмент + σ + α-окончания (-α/-ας/-ε) |
| Aor II | аугмент, без σ, окончания как imperf (-ον/-ες/-ε) но другой основа |
| Perf | удвоение (κ-перфект), окончания -κα/-κας/-κε |
| Pluperf | аугмент ε + удвоение + окончания -κειν/-κεις/-κει/-κεσαν |
| FutPerf | удвоение + σ + средние окончания (-σομαι) |

---

## Файлы

| Файл | Роль |
|------|------|
| `assets/greek_paradigms.db` | ОСНОВНАЯ БД (изменяется) |
| `assets/greek_paradigms.db.bak3` | Бэкап до всех исправлений |
| `assets/grammar-smyth-master/xhtml/greek_morphology.sqlite3` | Референс |
| `PARADIGM_AUDIT.md` | Чеклист всех лемм |
| `src/logic/paradigm.rs` | Rust: построение таблиц парадигм |
| `src/pages/study_build.rs` | Rust: логика квиза |
