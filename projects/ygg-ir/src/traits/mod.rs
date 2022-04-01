use std::collections::HashSet;

use yggdrasil_error::{Diagnostic, YggdrasilResult};

use crate::*;

mod field_descriptor;

pub enum FieldCount {}

pub trait FieldDescriptor {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>);
}

pub trait CodeOptimizer {
    fn optimize(&mut self, info: &GrammarInfo) -> YggdrasilResult<GrammarInfo>;
}

pub trait CodeGenerator<T> {
    fn generate(&mut self, info: &GrammarInfo) -> YggdrasilResult<T>;
}

impl GrammarInfo {
    pub fn optimize(&self, mut pass: Vec<impl CodeOptimizer>) -> YggdrasilResult<GrammarInfo> {
        let mut errors = vec![];
        let mut out = GrammarInfo::default();
        for co in pass.iter_mut() {
            let step = co.optimize(self)?;
            out = step.success;
            errors.extend(step.errors);
        }
        Ok(Diagnostic { success: out, errors })
    }
    pub fn codegen<T>(&self, mut pass: impl CodeGenerator<T>) -> YggdrasilResult<T> {
        pass.generate(self)
    }
}
