use super::*;
use yggdrasil_error::Validation;

/// Automatically insert or remove tags
#[derive(Default)]
pub struct RemarkTags {}

impl CodeOptimizer for RemarkTags {
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let mut info = info.clone();
        for rule in info.rules.values_mut() {
            match &mut rule.body {
                None => {}
                Some(s) => self.remark(s, true),
            }
        }

        Validation::Success { value: info, diagnostics: vec![] }
    }
}

impl RemarkTags {
    fn remark(&self, expr: &mut YggdrasilExpression, scope: bool) {
        let mark = if expr.remark { !scope } else { scope };
        expr.remark = false;
        match &mut expr.body {
            // ^(a b c)
            ExpressionBody::Concat(v) => {
                for item in &mut v.sequence {
                    self.remark(item, mark)
                }
            }
            // ^(a|b|c)
            ExpressionBody::Choice(v) => {
                for item in &mut v.branches {
                    self.remark(item, mark)
                }
            }
            // ^(a+)
            ExpressionBody::Unary(v) => self.remark(&mut v.base, mark),
            ExpressionBody::Rule(v) if mark => match &mut expr.tag {
                Some(_) => {}
                None => {
                    expr.tag = Some(v.name.clone());
                }
            },
            ExpressionBody::Text(_) => {}
            _ => {}
        }
    }
}
