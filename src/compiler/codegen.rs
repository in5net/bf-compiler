use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    types::BasicTypeEnum,
    types::IntType,
    values::{BasicValueEnum, FunctionValue, IntValue, PointerValue},
    AddressSpace, IntPredicate,
};

use crate::Node;

pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: Builder<'ctx>,
    function: FunctionValue<'ctx>,
    cell_type: IntType<'ctx>,
    index: PointerValue<'ctx>,
    array: PointerValue<'ctx>,
    putchar: FunctionValue<'ctx>,
    getchar: FunctionValue<'ctx>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub fn new(
        filename: &str,
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: Builder<'ctx>,
    ) -> Self {
        module.set_source_file_name(filename);

        let i32_type = context.i32_type();
        let str_type = context.i8_type().ptr_type(AddressSpace::Generic);

        let fn_type = i32_type.fn_type(
            &[
                BasicTypeEnum::IntType(i32_type),
                BasicTypeEnum::PointerType(str_type),
            ],
            false,
        );
        let function = module.add_function("main", fn_type, None);
        let block = context.append_basic_block(function, "body");
        builder.position_at_end(block);

        let int_type = context.i32_type();

        let size: u32 = 30_000;
        let array_ptr =
            builder.build_array_alloca(int_type, int_type.const_int(size as u64, false), "array");

        let index_ptr = builder.build_alloca(int_type, "index");
        builder.build_store(index_ptr, int_type.const_int(0, false));

        Self {
            context: &context,
            module: &module,
            builder,
            function,
            cell_type: int_type,
            index: index_ptr,
            array: array_ptr,
            putchar: Codegen::add_putchar(&context, &module),
            getchar: Codegen::add_getchar(&context, &module),
        }
    }

    fn add_putchar(context: &'ctx Context, module: &'a Module<'ctx>) -> FunctionValue<'ctx> {
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[BasicTypeEnum::IntType(i32_type)], false);
        module.add_function("putchar", fn_type, Some(Linkage::External))
    }
    fn add_getchar(context: &'ctx Context, module: &'a Module<'ctx>) -> FunctionValue<'ctx> {
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        module.add_function("getchar", fn_type, Some(Linkage::External))
    }

    pub fn generate_llvm_ir(&mut self, ast: Vec<Node>) {
        for node in ast {
            self.visit(node);
        }
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
    }

    fn get_cell_ptr(&self) -> PointerValue<'ctx> {
        unsafe {
            self.builder
                .build_gep(self.array, &[self.get_index()], "gep")
        }
    }

    fn get_cell(&self) -> IntValue<'ctx> {
        let ptr = self.get_cell_ptr();
        self.builder.build_load(ptr, "load_cell").into_int_value()
    }

    fn get_index(&self) -> IntValue<'ctx> {
        self.builder
            .build_load(self.index, "load_index")
            .into_int_value()
    }

    fn visit(&mut self, node: Node) {
        match node {
            Node::Left => {
                self.builder.build_store(
                    self.index,
                    self.builder.build_int_sub(
                        self.get_index(),
                        self.cell_type.const_int(1, false),
                        "left",
                    ),
                );
            }
            Node::Right => {
                self.builder.build_store(
                    self.index,
                    self.builder.build_int_add(
                        self.get_index(),
                        self.cell_type.const_int(1, false),
                        "right",
                    ),
                );
            }
            Node::Inc => {
                self.builder.build_store(
                    self.get_cell_ptr(),
                    self.builder.build_int_add(
                        self.get_cell(),
                        self.cell_type.const_int(1, false),
                        "dec",
                    ),
                );
            }
            Node::Dec => {
                self.builder.build_store(
                    self.get_cell_ptr(),
                    self.builder.build_int_sub(
                        self.get_cell(),
                        self.cell_type.const_int(1, false),
                        "dec",
                    ),
                );
            }
            Node::Output => {
                let value = self.get_cell();
                self.builder.build_call(
                    self.putchar,
                    &[BasicValueEnum::IntValue(value)],
                    "putchar",
                );
            }
            Node::Input => {
                let value = self
                    .builder
                    .build_call(self.getchar, &[], "getchar")
                    .try_as_basic_value()
                    .left()
                    .unwrap();
                self.builder.build_store(self.get_cell_ptr(), value);
            }
            Node::Group(body) => {
                let condition_block = self.context.append_basic_block(self.function, "group_cond");
                self.builder.build_unconditional_branch(condition_block);
                self.builder.position_at_end(condition_block);

                let condition_value = self.builder.build_int_compare(
                    IntPredicate::NE,
                    self.get_cell(),
                    self.cell_type.const_zero(),
                    "group_cmp",
                );

                let loop_block = self.context.append_basic_block(self.function, "group_body");
                self.builder.position_at_end(loop_block);
                for node in body {
                    self.visit(node);
                }

                let end_block = self.context.append_basic_block(self.function, "group_end");
                self.builder.build_unconditional_branch(condition_block);

                self.builder.position_at_end(condition_block);
                self.builder
                    .build_conditional_branch(condition_value, loop_block, end_block);

                self.builder.position_at_end(end_block);
            }
            Node::EOF => {}
        };
    }
}
