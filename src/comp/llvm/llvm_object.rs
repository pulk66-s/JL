pub trait LlvmObject {
    fn to_llvm_ir(&self) -> String;
}
