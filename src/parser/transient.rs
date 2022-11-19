use std::collections::HashMap;

use super::ast::{PsfAst, Trace};

pub struct TransientData {
    pub signals: HashMap<String, Vec<f64>>,
}

impl TransientData {
    pub fn from_ast(ast: &PsfAst) -> Self {
        // Assume all groups have count = 1
        // group name -> signal name
        let mut groups = HashMap::<&str, &str>::new();
        let mut i = 0;
        while i < ast.traces.len() {
            let (group, count) = if let Trace::Group { name, count } = ast.traces[i] {
                (name, count)
            } else {
                panic!("Incorrect group count");
            };

            assert_eq!(count, 1);
            i += 1;

            for _ in 0..count {
                if let Trace::Signal { name, .. } = ast.traces[i] {
                    groups.insert(group, name);
                } else {
                    panic!("Expected signal; found group");
                }
                i += 1;
            }
        }
        assert_eq!(groups.insert("time", "time"), None);
        let mut signals = HashMap::<String, Vec<f64>>::new();
        for v in ast.values.iter() {
            println!("signal = {:?}", v.signal);
            assert_eq!(v.values.len(), 1);
            if let Some(lst) = signals.get_mut(groups[v.signal]) {
                lst.push(v.values[0]);
            } else {
                signals.insert(groups[v.signal].to_string(), vec![v.values[0]]);
            }
        }

        Self { signals }
    }
}
