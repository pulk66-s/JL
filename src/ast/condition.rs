use super::AstExpr;

#[derive(Debug, Clone)]
pub struct ConditionAst {
    pub condition: Box<AstExpr>,
    pub true_expr: Vec<Box<AstExpr>>,
    pub false_expr: Option<Vec<Box<AstExpr>>>,
}

impl ConditionAst {
    pub fn new(
        condition: AstExpr,
        true_expr: Vec<AstExpr>,
        false_expr: Option<Vec<AstExpr>>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            true_expr: true_expr.into_iter().map(Box::new).collect(),
            false_expr: match false_expr {
                Some(e) => Some(e.into_iter().map(Box::new).collect()),
                None => None,
            },
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Condition: {}, true: {}, false: {}",
            self.condition.to_string(),
            self.true_expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            match &self.false_expr {
                Some(exprs) => exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                None => "None".to_string(),
            }
        )
    }
}
