use std::collections::{HashMap, HashSet};

use serde_json::value;

fn main() {
    const URL: &str = "https://raw.githubusercontent.com/lemonmanf/test/main/db.json";
    let json = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let value: value::Value = serde_json::from_str(&json).unwrap();
    let classes_by_db_idx = {
        let mut map = HashMap::new();
        value
            .get("classes")
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
            .for_each(|(class_name, value)| {
                // If the value does not have a "archetype" key, it is not combat class.
                if value.get("archetype").is_none() {
                    return;
                }
                ["native", "cross", "summon", "arcana", "iaijutsu"]
                    .iter()
                    .for_each(|key| {
                        if value.get(key).is_none() {
                            return;
                        }
                        value
                            .get(key)
                            .unwrap()
                            .as_array()
                            .unwrap()
                            .iter()
                            .for_each(|id| {
                                map.entry(id.as_u64().unwrap())
                                    .or_insert(HashSet::new())
                                    .insert(class_name);
                            })
                    })
            });
        map
    };
    let mut csv = String::new();
    csv.push_str("ClassName,ActionName,GameIdx,DBIdx\n");
    let mut rows: Vec<(&str, &str, u64, u64)> = vec![];
    value
        .get("skills")
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .for_each(|(db_idx, value)| {
            let db_idx = db_idx.parse::<u64>().unwrap();
            if !classes_by_db_idx.contains_key(&db_idx) {
                return;
            }
            let action_name = value.get("name").unwrap().as_str().unwrap();
            let game_idx = if let Some(c) = value.get("c") {
                c.as_str().unwrap().parse::<u64>().unwrap()
            } else {
                // If the value does not have a "c" key, game idx is not exist.
                0
            };
            let classes = classes_by_db_idx.get(&db_idx).unwrap();
            classes.iter().for_each(|class_name| {
                rows.push((class_name, action_name, game_idx, db_idx));
            });
        });
    rows.sort_by_key(|(class_name, _, _, db_idx)| (*class_name, *db_idx));
    rows.iter()
        .for_each(|(class_name, action_name, game_idx, db_idx)| {
            csv.push_str(&format!(
                "\"{}\",\"{}\",{},{}\n",
                class_name, action_name, game_idx, db_idx
            ));
        });
    print!("{}", csv);
}
