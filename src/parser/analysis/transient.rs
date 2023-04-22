use std::collections::HashMap;

use crate::bin_search_before;
use crate::parser::ast::{PsfAst, Trace, Values};

pub struct TransientData {
    pub signals: HashMap<String, Vec<f64>>,
    pub time: String,
}

impl TransientData {
    pub fn from_ast(ast: &PsfAst) -> Self {
        // Assume all groups have count = 1
        // group name -> signal name
        let mut groups = HashMap::<&str, &str>::new();
        let mut i = 0;
        while i < ast.traces.len() {
            match ast.traces[i] {
                Trace::Group { name: group, count } => {
                    debug_assert!(count >= 0);
                    let count = count as usize;
                    for j in 1..=count {
                        if let Trace::Signal { name, .. } = ast.traces[i + j] {
                            groups.insert(group, name);
                        } else {
                            panic!("Expected signal; found group");
                        }
                    }
                    i += count + 1;
                }
                Trace::Signal { name, .. } => {
                    groups.insert(name, name);
                    i += 1;
                }
            }
        }

        groups.insert("time", "time");
        let mut signals = HashMap::<String, Vec<f64>>::new();
        for v in ast.values.iter() {
            if let Values::Real(values) = &v.values {
                debug_assert_eq!(values.len(), 1);
                if let Some(lst) = signals.get_mut(groups[v.signal]) {
                    lst.push(values[0]);
                } else {
                    signals.insert(groups[v.signal].to_string(), vec![values[0]]);
                }
            } else {
                panic!("Expected real signal values; found complex");
            }
        }

        Self {
            signals,
            time: "time".to_string(),
        }
    }

    /// Gets the index into the data arrays
    /// corresponding to the latest time less than or equal to `t`.
    pub fn idx_before_time(&self, t: f64) -> Option<usize> {
        bin_search_before(self.signal(&self.time).unwrap(), t)
    }

    #[inline]
    pub fn signal(&self, name: &str) -> Option<&Vec<f64>> {
        self.signals.get(name)
    }
}
