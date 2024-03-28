use super::AstExpr;

#[derive(Debug, Clone)]
pub struct ConditionAst {
    pub condition: String,
    pub true_expr: Vec<Box<AstExpr>>,
    pub false_expr: Vec<Box<AstExpr>>,
}

impl ConditionAst {
    pub fn to_string(&self) -> String {
        format!(
            "Condition: {}, true: {}, false: {}",
            self.condition,
            self.true_expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.false_expr
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
