//! Reads `content/game.toml`, checks it, and renders the generated code.
//!
//! Pure functions only (text in, text out), so the generator binary and the
//! tests in `core/tests/content.rs` share them. No file access here.
//!
//! The content file uses a small subset of TOML: top-level `key = value`
//! lines and `[[table]]` entries holding `key = value` lines. Values are
//! integers, decimals, basic "strings", booleans and single-line arrays.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Paths relative to the repository root.
pub const CONTENT_PATH: &str = "content/game.toml";
pub const RUST_OUT: &str = "core/src/generated.rs";
pub const JS_OUT: &str = "src/generated.js";
pub const WGSL_PATH: &str = "src/shader.wgsl";

pub const WGSL_BEGIN: &str = "// BEGIN GENERATED from content/game.toml by tools/gen. Do not edit; run ./build.sh.";
pub const WGSL_END: &str = "// END GENERATED";

/// Image slots per part in the shader (`PartRects` in src/shader.wgsl and
/// the rect buffer in src/renderer.js). Part ids must fit.
pub const PART_SLOTS: u32 = 32;

// ------------------------------------------------------------------ parsing

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    /// A decimal, kept as written so generated literals match the source.
    Num(String),
    Str(String),
    Bool(bool),
    Array(Vec<Value>),
}

impl Value {
    fn kind(&self) -> &'static str {
        match self {
            Value::Int(_) => "an integer",
            Value::Num(_) => "a decimal",
            Value::Str(_) => "a string",
            Value::Bool(_) => "a boolean",
            Value::Array(_) => "an array",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub key: String,
    pub value: Value,
    pub line: usize,
}

/// The top-level keys (name "") or one `[[name]]` entry.
#[derive(Clone, Debug)]
pub struct Table {
    pub name: String,
    pub line: usize,
    pub entries: Vec<Entry>,
}

/// Parse the TOML subset. The first table is the top level.
pub fn parse(src: &str) -> Result<Vec<Table>, String> {
    let mut tables = vec![Table { name: String::new(), line: 0, entries: Vec::new() }];
    for (i, raw) in src.lines().enumerate() {
        let line = i + 1;
        let text = raw.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        if let Some(rest) = text.strip_prefix("[[") {
            let end = rest.find("]]").ok_or(format!("line {line}: missing ]] in table header"))?;
            let name = rest[..end].trim();
            let after = rest[end + 2..].trim();
            if !after.is_empty() && !after.starts_with('#') {
                return Err(format!("line {line}: unexpected text after table header"));
            }
            if !is_name(name) {
                return Err(format!("line {line}: bad table name `{name}`"));
            }
            tables.push(Table { name: name.to_string(), line, entries: Vec::new() });
            continue;
        }
        if text.starts_with('[') {
            return Err(format!("line {line}: only [[table]] headers are supported"));
        }
        let eq = text.find('=').ok_or(format!("line {line}: expected `key = value`"))?;
        let key = text[..eq].trim();
        if !is_name(key) {
            return Err(format!("line {line}: bad key `{key}`"));
        }
        let mut cur = Cursor { s: &text.as_bytes()[eq + 1..], i: 0, line };
        cur.skip_ws();
        let value = cur.value()?;
        cur.skip_ws();
        if cur.i < cur.s.len() && cur.s[cur.i] != b'#' {
            return Err(format!("line {line}: unexpected text after value"));
        }
        let table = tables.last_mut().unwrap();
        if table.entries.iter().any(|e| e.key == key) {
            return Err(format!("line {line}: duplicate key `{key}`"));
        }
        table.entries.push(Entry { key: key.to_string(), value, line });
    }
    Ok(tables)
}

struct Cursor<'a> {
    s: &'a [u8],
    i: usize,
    line: usize,
}

impl Cursor<'_> {
    fn skip_ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] == b' ' || self.s[self.i] == b'\t') {
            self.i += 1;
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        let line = self.line;
        match self.s.get(self.i) {
            None => Err(format!("line {line}: missing value")),
            Some(b'"') => self.string(),
            Some(b'[') => {
                self.i += 1;
                let mut items = Vec::new();
                loop {
                    self.skip_ws();
                    if self.s.get(self.i) == Some(&b']') {
                        self.i += 1;
                        return Ok(Value::Array(items));
                    }
                    items.push(self.value()?);
                    self.skip_ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {}
                        _ => return Err(format!("line {line}: expected , or ] in array")),
                    }
                }
            }
            Some(_) => {
                let start = self.i;
                while self.i < self.s.len() && !b" \t,]#".contains(&self.s[self.i]) {
                    self.i += 1;
                }
                let word = std::str::from_utf8(&self.s[start..self.i]).unwrap();
                match word {
                    "true" => Ok(Value::Bool(true)),
                    "false" => Ok(Value::Bool(false)),
                    _ if word.contains(['.', 'e', 'E']) && word.parse::<f64>().is_ok() => {
                        Ok(Value::Num(word.to_string()))
                    }
                    _ => word
                        .parse::<i64>()
                        .map(Value::Int)
                        .map_err(|_| format!("line {line}: cannot read value `{word}`")),
                }
            }
        }
    }

    fn string(&mut self) -> Result<Value, String> {
        let line = self.line;
        self.i += 1;
        let mut out = Vec::new();
        loop {
            match self.s.get(self.i) {
                None => return Err(format!("line {line}: unterminated string")),
                Some(b'"') => {
                    self.i += 1;
                    return Ok(Value::Str(String::from_utf8(out).unwrap()));
                }
                Some(b'\\') => {
                    let c = match self.s.get(self.i + 1) {
                        Some(b'"') => b'"',
                        Some(b'\\') => b'\\',
                        _ => return Err(format!("line {line}: only \\\" and \\\\ escapes are supported")),
                    };
                    out.push(c);
                    self.i += 2;
                }
                Some(&c) => {
                    out.push(c);
                    self.i += 1;
                }
            }
        }
    }
}

/// Lowercase snake_case: a letter, then letters, digits or underscores.
fn is_name(s: &str) -> bool {
    let mut b = s.bytes();
    matches!(b.next(), Some(b'a'..=b'z')) && b.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
}

// ------------------------------------------------------------------ content

#[derive(Clone, Debug)]
pub struct Part {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Upgrade {
    pub id: u32,
    pub name: String,
    pub max: u32,
    pub glyph: String,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Mission {
    pub id: u32,
    pub name: String,
    /// What the mission asks for, in its own unit (seconds for `survive`, enemies for `kills`). 0 for none.
    pub goal: u32,
}

#[derive(Clone, Debug)]
pub struct StateField {
    pub name: String,
    pub index: u32,
    /// Some for a run of floats, None for a single float.
    pub len: Option<u32>,
}

/// Checked content, each list sorted by id (state by index).
#[derive(Clone, Debug)]
pub struct Content {
    pub ink: [String; 3],
    pub state_floats: u32,
    pub parts: Vec<Part>,
    pub upgrades: Vec<Upgrade>,
    pub events: Vec<Event>,
    pub missions: Vec<Mission>,
    pub state: Vec<StateField>,
}

/// Reads typed fields out of one table and collects errors.
struct Fields<'a> {
    t: &'a Table,
    used: Vec<&'a str>,
    errs: &'a mut Vec<String>,
}

impl<'a> Fields<'a> {
    fn get(&mut self, key: &'a str) -> Option<&'a Entry> {
        self.used.push(key);
        self.t.entries.iter().find(|e| e.key == key)
    }

    fn where_(&self) -> String {
        if self.t.name.is_empty() {
            "top level".to_string()
        } else {
            format!("[[{}]] at line {}", self.t.name, self.t.line)
        }
    }

    fn missing(&mut self, key: &str) {
        let w = self.where_();
        self.errs.push(format!("{w}: missing `{key}`"));
    }

    fn wrong(&mut self, e: &Entry, want: &str) {
        self.errs.push(format!("line {}: `{}` must be {want}, not {}", e.line, e.key, e.value.kind()));
    }

    fn uint(&mut self, key: &'a str) -> Option<u32> {
        let e = match self.get(key) {
            Some(e) => e,
            None => {
                self.missing(key);
                return None;
            }
        };
        match e.value {
            Value::Int(v) if (0..=u32::MAX as i64).contains(&v) => Some(v as u32),
            _ => {
                self.wrong(e, "a whole number of 0 or more");
                None
            }
        }
    }

    fn string(&mut self, key: &'a str) -> Option<String> {
        let e = match self.get(key) {
            Some(e) => e,
            None => {
                self.missing(key);
                return None;
            }
        };
        match &e.value {
            Value::Str(s) => Some(s.clone()),
            _ => {
                self.wrong(e, "a string");
                None
            }
        }
    }

    fn name(&mut self) -> Option<String> {
        let line = self.t.entries.iter().find(|e| e.key == "name").map_or(self.t.line, |e| e.line);
        let n = self.string("name")?;
        if !is_name(&n) {
            self.errs.push(format!("line {line}: name `{n}` must be lowercase snake_case"));
            return None;
        }
        Some(n)
    }

    /// Report keys this table has but nothing read.
    fn finish(self) {
        for e in &self.t.entries {
            if !self.used.contains(&e.key.as_str()) {
                self.errs.push(format!("line {}: unknown key `{}`", e.line, e.key));
            }
        }
    }
}

/// Table names the content file may use, also valid as a `len` reference.
const TABLES: [&str; 5] = ["part", "upgrade", "event", "mission", "state"];

/// Parse and check the content file. Returns every problem found.
pub fn load(src: &str) -> Result<Content, Vec<String>> {
    let tables = parse(src).map_err(|e| vec![e])?;
    let mut errs = Vec::new();

    // Top level.
    let mut f = Fields { t: &tables[0], used: Vec::new(), errs: &mut errs };
    let mut ink = None;
    match f.get("ink") {
        None => f.missing("ink"),
        Some(e) => match &e.value {
            Value::Array(items) if items.len() == 3 => {
                let mut out: Vec<String> = Vec::new();
                for v in items {
                    let text = match v {
                        Value::Int(i) => i.to_string(),
                        Value::Num(s) => s.clone(),
                        _ => break,
                    };
                    match text.parse::<f64>() {
                        Ok(x) if (0.0..=1.0).contains(&x) => out.push(text),
                        _ => break,
                    }
                }
                if out.len() == 3 {
                    ink = Some([out[0].clone(), out[1].clone(), out[2].clone()]);
                } else {
                    f.errs.push(format!("line {}: `ink` values must be numbers from 0 to 1", e.line));
                }
            }
            _ => f.wrong(e, "an array of 3 numbers"),
        },
    }
    let state_floats = f.uint("state_floats");
    f.finish();

    let mut parts = Vec::new();
    let mut upgrades = Vec::new();
    let mut events = Vec::new();
    let mut missions = Vec::new();
    // (name, index, len given as written, line)
    let mut raw_state: Vec<(String, u32, Option<Value>, usize)> = Vec::new();

    for t in &tables[1..] {
        let mut f = Fields { t, used: Vec::new(), errs: &mut errs };
        match t.name.as_str() {
            "part" => {
                if let (Some(id), Some(name)) = (f.uint("id"), f.name()) {
                    if id >= PART_SLOTS {
                        f.errs.push(format!("line {}: part id {id} must be below {PART_SLOTS}", t.line));
                    }
                    parts.push((Part { id, name }, t.line));
                }
            }
            "upgrade" => {
                let (id, name, max, glyph) = (f.uint("id"), f.name(), f.uint("max"), f.string("glyph"));
                if let (Some(id), Some(name), Some(max), Some(glyph)) = (id, name, max, glyph) {
                    if !(1..=255).contains(&max) {
                        f.errs.push(format!("line {}: upgrade `max` must be 1 to 255", t.line));
                    }
                    if glyph.chars().count() != 1 {
                        f.errs.push(format!("line {}: upgrade `glyph` must be one character", t.line));
                    }
                    upgrades.push((Upgrade { id, name, max, glyph }, t.line));
                }
            }
            "event" => {
                if let (Some(id), Some(name)) = (f.uint("id"), f.name()) {
                    if id == 0 {
                        f.errs.push(format!("line {}: event id 0 is reserved for no event", t.line));
                    }
                    events.push((Event { id, name }, t.line));
                }
            }
            "mission" => {
                if let (Some(id), Some(name), Some(goal)) = (f.uint("id"), f.name(), f.uint("goal")) {
                    missions.push((Mission { id, name, goal }, t.line));
                }
            }
            "state" => {
                let len = f.get("len").map(|e| e.value.clone());
                if let (Some(name), Some(index)) = (f.name(), f.uint("index")) {
                    raw_state.push((name, index, len, t.line));
                }
            }
            other => {
                f.errs.push(format!("line {}: unknown table [[{other}]]", t.line));
                f.used.extend(t.entries.iter().map(|e| e.key.as_str()));
            }
        }
        f.finish();
    }

    // Unique ids and names within each table.
    fn unique<'a>(errs: &mut Vec<String>, what: &str, items: impl Iterator<Item = (u32, &'a str, usize)> + Clone) {
        let mut ids: BTreeMap<u32, usize> = BTreeMap::new();
        let mut names: BTreeMap<&str, usize> = BTreeMap::new();
        for (id, name, line) in items {
            if let Some(first) = ids.insert(id, line) {
                errs.push(format!("line {line}: duplicate {what} id {id} (first at line {first})"));
            }
            if let Some(first) = names.insert(name, line) {
                errs.push(format!("line {line}: duplicate {what} name `{name}` (first at line {first})"));
            }
        }
    }
    unique(&mut errs, "part", parts.iter().map(|(p, l)| (p.id, p.name.as_str(), *l)));
    unique(&mut errs, "upgrade", upgrades.iter().map(|(u, l)| (u.id, u.name.as_str(), *l)));
    unique(&mut errs, "event", events.iter().map(|(e, l)| (e.id, e.name.as_str(), *l)));
    unique(&mut errs, "mission", missions.iter().map(|(m, l)| (m.id, m.name.as_str(), *l)));
    let mut state_names: BTreeMap<&str, usize> = BTreeMap::new();
    for (name, _, _, line) in &raw_state {
        if let Some(first) = state_names.insert(name, *line) {
            errs.push(format!("line {line}: duplicate state name `{name}` (first at line {first})"));
        }
    }

    // Upgrade ids index tables, so they must be 0..n without gaps.
    let mut up_ids: Vec<u32> = upgrades.iter().map(|(u, _)| u.id).collect();
    up_ids.sort_unstable();
    up_ids.dedup();
    if up_ids.iter().enumerate().any(|(i, id)| i as u32 != *id) {
        errs.push(format!("upgrade ids must run 0 to {} without gaps", upgrades.len().saturating_sub(1)));
    }

    // Mission ids index tables too.
    let mut mission_ids: Vec<u32> = missions.iter().map(|(m, _)| m.id).collect();
    mission_ids.sort_unstable();
    mission_ids.dedup();
    if mission_ids.iter().enumerate().any(|(i, id)| i as u32 != *id) {
        errs.push(format!("mission ids must run 0 to {} without gaps", missions.len().saturating_sub(1)));
    }
    if missions.is_empty() {
        errs.push("at least one [[mission]] is needed".to_string());
    }

    // State fields: resolve `len`, then check bounds and overlap.
    let count = |table: &str| match table {
        "part" => parts.len(),
        "upgrade" => upgrades.len(),
        "event" => events.len(),
        "mission" => missions.len(),
        _ => raw_state.len(),
    };
    let mut state = Vec::new();
    for (name, index, len, line) in &raw_state {
        let len = match len {
            None => None,
            Some(Value::Int(n)) if *n >= 1 && *n <= u32::MAX as i64 => Some(*n as u32),
            Some(Value::Str(table)) if TABLES.contains(&table.as_str()) => Some(count(table) as u32),
            Some(Value::Str(table)) => {
                errs.push(format!("line {line}: state `{name}` has len \"{table}\", which is not a defined table"));
                continue;
            }
            Some(v) => {
                errs.push(format!(
                    "line {line}: state `{name}` len must be a number of 1 or more or a table name, not {}",
                    v.kind()
                ));
                continue;
            }
        };
        state.push((StateField { name: name.clone(), index: *index, len }, *line));
    }
    state.sort_by_key(|(s, _)| s.index);
    let mut end = 0u64;
    let mut prev = "";
    for (s, line) in &state {
        let n = s.len.unwrap_or(1) as u64;
        if (s.index as u64) < end {
            errs.push(format!("line {line}: state `{}` at {} overlaps `{prev}`", s.name, s.index));
        }
        end = end.max(s.index as u64 + n);
        prev = &s.name;
        if let Some(total) = state_floats {
            if s.index as u64 + n > total as u64 {
                errs.push(format!("line {line}: state `{}` ends past state_floats ({total})", s.name));
            }
        }
    }

    if !errs.is_empty() {
        return Err(errs);
    }
    let mut parts: Vec<Part> = parts.into_iter().map(|(p, _)| p).collect();
    let mut upgrades: Vec<Upgrade> = upgrades.into_iter().map(|(u, _)| u).collect();
    let mut events: Vec<Event> = events.into_iter().map(|(e, _)| e).collect();
    parts.sort_by_key(|p| p.id);
    upgrades.sort_by_key(|u| u.id);
    events.sort_by_key(|e| e.id);
    let mut missions: Vec<Mission> = missions.into_iter().map(|(m, _)| m).collect();
    missions.sort_by_key(|m| m.id);
    Ok(Content {
        ink: ink.unwrap(),
        state_floats: state_floats.unwrap(),
        parts,
        upgrades,
        events,
        missions,
        state: state.into_iter().map(|(s, _)| s).collect(),
    })
}

// ---------------------------------------------------------------- rendering

fn upper(name: &str) -> String {
    name.to_ascii_uppercase()
}

/// The constant suffix for a state field: runs of floats get a 0 suffix.
fn state_key(s: &StateField) -> String {
    if s.len.is_some() {
        format!("{}0", upper(&s.name))
    } else {
        upper(&s.name)
    }
}

/// A decimal literal for Rust and WGSL: `1` becomes `1.0`.
fn float_lit(s: &str) -> String {
    if s.contains(['.', 'e', 'E']) {
        s.to_string()
    } else {
        format!("{s}.0")
    }
}

/// `core/src/generated.rs`.
pub fn render_rust(c: &Content) -> String {
    let mut o = String::new();
    o.push_str("//! Generated from `content/game.toml` by `tools/gen`. Do not edit: change the\n");
    o.push_str("//! content file and run `./build.sh`. Re-exported by `crate::defs`.\n\n");

    o.push_str("// Part ids.\n");
    for p in &c.parts {
        writeln!(o, "pub const P_{}: f32 = {}.0;", upper(&p.name), p.id).unwrap();
    }

    o.push_str("\n// Upgrade ids.\n");
    for u in &c.upgrades {
        writeln!(o, "pub const U_{}: usize = {};", upper(&u.name), u.id).unwrap();
    }
    writeln!(o, "pub const N_UPGRADES: usize = {};", c.upgrades.len()).unwrap();
    let maxes: Vec<String> = c.upgrades.iter().map(|u| u.max.to_string()).collect();
    writeln!(o, "pub const UPGRADE_MAX: [u8; N_UPGRADES] = [{}];", maxes.join(", ")).unwrap();

    o.push_str("\n// Event types, read by the host for sound.\n");
    for e in &c.events {
        writeln!(o, "pub const EV_{}: f32 = {}.0;", upper(&e.name), e.id).unwrap();
    }

    o.push_str("\n// Mission ids, and what each one asks for (seconds for `survive`, enemies for `kills`).\n");
    for m in &c.missions {
        writeln!(o, "pub const M_{}: usize = {};", upper(&m.name), m.id).unwrap();
    }
    writeln!(o, "pub const N_MISSIONS: usize = {};", c.missions.len()).unwrap();
    let goals: Vec<String> = c.missions.iter().map(|m| format!("{}.0", m.goal)).collect();
    writeln!(o, "pub const MISSION_GOAL: [f32; N_MISSIONS] = [{}];", goals.join(", ")).unwrap();

    o.push_str("\n// HUD state buffer: size and the index of each field.\n");
    writeln!(o, "pub const STATE_FLOATS: usize = {};", c.state_floats).unwrap();
    for s in &c.state {
        writeln!(o, "pub const S_{}: usize = {};", state_key(s), s.index).unwrap();
    }

    o.push('\n');
    let ink: Vec<String> = c.ink.iter().map(|v| float_lit(v)).collect();
    writeln!(o, "pub const INK: [f32; 3] = [{}];", ink.join(", ")).unwrap();
    o
}

fn js_str(s: &str) -> String {
    format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
}

/// `src/generated.js`.
pub fn render_js(c: &Content) -> String {
    let mut o = String::new();
    o.push_str("// Generated from content/game.toml by tools/gen. Do not edit: change the\n");
    o.push_str("// content file and run ./build.sh. Imported through src/data.js.\n\n");

    o.push_str("/** Part ids by name. A name is also the key for an image override in assets/parts.json. */\n");
    o.push_str("export const PART_IDS = {\n");
    for p in &c.parts {
        writeln!(o, "  {}: {},", p.name, p.id).unwrap();
    }
    o.push_str("};\n\n");

    o.push_str("/** Upgrades, indexed by id. `glyph` is the brush mark shown on cards and HUD chips.\n");
    o.push_str(" * Names and descriptions are in src/i18n.js as upgrade.<id>.name and .desc. */\n");
    o.push_str("export const UPGRADES = [\n");
    for u in &c.upgrades {
        writeln!(o, "  {{ name: {}, glyph: {}, max: {} }},", js_str(&u.name), js_str(&u.glyph), u.max).unwrap();
    }
    o.push_str("];\n\n");

    o.push_str("/** Event types reported by the core. */\n");
    o.push_str("export const EVENT = {\n");
    for e in &c.events {
        writeln!(o, "  {}: {},", upper(&e.name), e.id).unwrap();
    }
    o.push_str("};\n\n");

    o.push_str(
        "/** Missions, indexed by id. `goal` is what the mission asks for (seconds for survive, enemies for kills).\n",
    );
    o.push_str(" * Names and descriptions are in src/i18n.js as mission.<name>.name and .desc. */\n");
    o.push_str("export const MISSIONS = [\n");
    for m in &c.missions {
        writeln!(o, "  {{ id: {}, name: {}, goal: {} }},", m.id, js_str(&m.name), m.goal).unwrap();
    }
    o.push_str("];\n\n");

    o.push_str("/** Mission ids by name. */\n");
    o.push_str("export const MISSION_IDS = {\n");
    for m in &c.missions {
        writeln!(o, "  {}: {},", m.name, m.id).unwrap();
    }
    o.push_str("};\n\n");

    o.push_str("/** Size of the state array written by the core, in floats. */\n");
    writeln!(o, "export const STATE_FLOATS = {};", c.state_floats).unwrap();
    o.push('\n');
    o.push_str("/** Indices into the state array. A run of floats has a 0 suffix (CHOICE0). */\n");
    o.push_str("export const STATE = {\n");
    for s in &c.state {
        writeln!(o, "  {}: {},", state_key(s), s.index).unwrap();
    }
    o.push_str("};\n\n");

    writeln!(o, "export const INK = [{}];", c.ink.join(", ")).unwrap();
    o
}

/// The lines between the markers in `src/shader.wgsl`, markers included.
pub fn render_wgsl_block(c: &Content) -> String {
    let mut o = String::new();
    o.push_str(WGSL_BEGIN);
    o.push('\n');
    let ink: Vec<String> = c.ink.iter().map(|v| float_lit(v)).collect();
    writeln!(o, "const INK = vec3f({});", ink.join(", ")).unwrap();
    for p in &c.parts {
        writeln!(o, "const PART_{}: i32 = {};", upper(&p.name), p.id).unwrap();
    }
    o.push_str(WGSL_END);
    o.push('\n');
    o
}

/// Replace the generated block in the shader source. Line endings become LF.
pub fn splice_wgsl(shader: &str, c: &Content) -> Result<String, String> {
    let src = shader.replace("\r\n", "\n");
    let begin = src.find(WGSL_BEGIN).ok_or(format!("{WGSL_PATH}: no begin marker `{WGSL_BEGIN}`"))?;
    if src[begin + 1..].contains(WGSL_BEGIN) {
        return Err(format!("{WGSL_PATH}: more than one begin marker"));
    }
    let end_rel = src[begin..].find(WGSL_END).ok_or(format!("{WGSL_PATH}: no end marker `{WGSL_END}`"))?;
    let mut end = begin + end_rel + WGSL_END.len();
    if src[end..].starts_with('\n') {
        end += 1;
    }
    if !src[..begin].is_empty() && !src[..begin].ends_with('\n') {
        return Err(format!("{WGSL_PATH}: the begin marker must start a line"));
    }
    Ok(format!("{}{}{}", &src[..begin], render_wgsl_block(c), &src[end..]))
}
