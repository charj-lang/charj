use inkwell::context::Context;

use crate::base_target::BaseTarget;
use crate::lowerify::code_object::CodeObject;
use crate::Namespace;

pub struct ClassicTarget {}

impl ClassicTarget {
    pub fn build<'a>(
        filename: &'a String,
        context: &'a Context,
        ns: &'a Namespace,
    ) -> CodeObject<'a> {
        let target = ClassicTarget {};

        let mut structure = CodeObject::new(context, filename, ns, "x86_64");
        // todo: call main after build others.
        for cfg in &ns.cfgs {
            target.emit_function(&mut structure, &cfg);
        }

        structure
    }
}

impl<'a> BaseTarget<'a> for ClassicTarget {}
