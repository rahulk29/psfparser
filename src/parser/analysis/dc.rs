use std::collections::HashMap;

use crate::parser::ast::{PsfAst, Trace, Values};

pub enum DcData {
    Op(OpData),
    Sweep(SweepData),
}

pub struct OpData {
    pub signals: HashMap<String, f64>,
}

pub struct SweepData {
    pub signals: HashMap<String, Vec<f64>>,
    pub param: (String, Vec<f64>),
}

impl DcData {
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

        let sweepvar = if ast.sweeps.is_empty() {
            None
        } else {
            debug_assert_eq!(ast.sweeps.len(), 1);
            Some(ast.sweeps[0].name.to_string())
        };
        let mut signals = HashMap::<String, Vec<f64>>::new();
        let mut sweepvals = Vec::<f64>::new();
        for v in ast.values.iter() {
            if let Values::Real(values) = &v.values {
                debug_assert_eq!(values.len(), 1);
                if Some(v.signal) == sweepvar.as_deref() {
                    sweepvals.push(values[0]);
                } else {
                    let group = groups.get(v.signal).unwrap_or(&v.signal);
                    if let Some(lst) = signals.get_mut(*group) {
                        lst.push(values[0]);
                    } else {
                        signals.insert(group.to_string(), vec![values[0]]);
                    }
                }
            } else {
                panic!("Expected real signal values; found complex");
            }
        }

        match sweepvar {
            Some(name) => Self::Sweep(SweepData {
                signals,
                param: (name, sweepvals),
            }),
            None => Self::Op(OpData {
                signals: HashMap::from_iter(signals.into_iter().map(|(k, v)| (k, v[0]))),
            }),
        }
    }
}

impl OpData {
    #[inline]
    pub fn signal(&self, name: &str) -> Option<f64> {
        self.signals.get(name).cloned()
    }
}

impl SweepData {
    #[inline]
    pub fn signal(&self, name: &str) -> Option<&Vec<f64>> {
        self.signals.get(name)
    }
}
