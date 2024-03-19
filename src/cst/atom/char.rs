use crate::cst::node::Node;

pub struct CharAtom {
    pub value: char
}

impl Node for CharAtom {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
