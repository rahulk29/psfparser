use std::cmp::Ordering;
use std::collections::HashMap;

use super::ast::{PsfAst, Trace};

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
                    assert!(count >= 0);
                    let count = count as usize;
                    for j in 1..=count {
                        if let Trace::Signal { name, .. } = ast.traces[i+j] {
                            groups.insert(group, name);
                        } else {
                            panic!("Expected signal; found group");
                        }
                    }
                    i += count + 1;
                },
                Trace::Signal { name, .. } => {
                    groups.insert(name, name);
                    i += 1;
                }
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

fn bin_search_before(data: &[f64], target: f64) -> Option<usize> {
    if data.is_empty() {
        return None;
    }

    let mut ans = None;
    let mut lo = 0usize;
    let mut hi = data.len() - 1;
    let mut x;
    while lo < hi {
        let mid = (lo + hi) / 2;
        x = data[mid];
        match target.total_cmp(&x) {
            Ordering::Less => hi = mid - 1,
            Ordering::Greater => {
                lo = mid + 1;
                ans = Some(mid)
            }
            Ordering::Equal => return Some(mid),
        }
    }

    ans
}
