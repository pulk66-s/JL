use crate::cst::node::Node;

pub struct StringAtom {
    pub value: String
}

impl Node for StringAtom {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
