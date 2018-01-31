use {CmpOp, Instruction, Op};
use cretonne::ir::{AbiParam, CallConv, ExternalName, Function, InstBuilder, Signature};
use cretonne::ir::types::I64;
use cretonne::ir::entities::JumpTable;
use cretonne::entity::EntityRef;
use cton_frontend::{FunctionBuilder, ILBuilder};
use cretonne::ir::condcodes::IntCC;
use cretonne::settings::Flags;
use cretonne::Context;
use cton_native::builders;
use std::usize;
use std::collections::{HashMap, HashSet};
use mmap::{MapOption, MemoryMap};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Variable(usize);
impl EntityRef for Variable {
    fn new(index: usize) -> Self {
        Variable(index)
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

pub fn ir_compile_part1(instructions: &[Instruction]) -> Function {
    let mut variables = HashSet::new();
    for instruction in instructions {
        variables.insert(instruction.src);
        variables.insert(instruction.dst);
    }
    let variables = variables
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let ret_var = Variable(variables.len());

    let mut sig = Signature::new(CallConv::Native);
    sig.returns.push(AbiParam::new(I64));
    let mut il_builder = ILBuilder::<Variable>::new();
    let mut func = Function::with_name_signature(ExternalName::user(0, 0), sig);
    {
        let mut builder = FunctionBuilder::<Variable>::new(&mut func, &mut il_builder);

        let blocks = (0..instructions.len() + variables.len() + 1)
            .map(|_| builder.create_ebb())
            .collect::<Vec<_>>();

        for i in 0..variables.len() {
            builder.declare_var(Variable(i), I64);
        }

        builder.declare_var(ret_var, I64);

        let mut block_idx = 0;

        builder.switch_to_block(blocks[block_idx]);
        builder.seal_block(blocks[block_idx]);

        for i in 0..variables.len() {
            let tmp = builder.ins().iconst(I64, 0);
            builder.def_var(Variable(i), tmp);
        }

        let tmp = builder.ins().iconst(I64, 0);
        builder.def_var(ret_var, tmp);

        for instruction in instructions {
            block_idx += 1;

            let arg1 = builder.use_var(Variable(variables[instruction.src]));
            let arg2 = builder.ins().iconst(I64, instruction.cmp_val);
            let cond = match instruction.cmp_op {
                CmpOp::Eq => IntCC::Equal,
                CmpOp::Ne => IntCC::NotEqual,
                CmpOp::Lt => IntCC::SignedLessThan,
                CmpOp::Le => IntCC::SignedLessThanOrEqual,
                CmpOp::Gt => IntCC::SignedGreaterThan,
                CmpOp::Ge => IntCC::SignedGreaterThanOrEqual,
            };

            let tmp = builder.ins().icmp(cond, arg1, arg2);
            builder.ins().brz(tmp, blocks[block_idx], &[]);

            let dst = Variable(variables[instruction.dst]);
            let arg1 = builder.use_var(dst);
            let arg2 = builder.ins().iconst(I64, instruction.val);

            let tmp = if instruction.op == Op::Inc {
                builder.ins().iadd(arg1, arg2)
            } else {
                builder.ins().isub(arg1, arg2)
            };
            builder.def_var(dst, tmp);

            builder.ins().jump(blocks[block_idx], &[]);

            builder.switch_to_block(blocks[block_idx]);
            builder.seal_block(blocks[block_idx]);
        }

        for i in 0..variables.len() {
            block_idx += 1;

            let arg1 = builder.use_var(Variable(i));
            let arg2 = builder.use_var(ret_var);

            let tmp = builder.ins().icmp(IntCC::SignedGreaterThan, arg1, arg2);
            builder.ins().brz(tmp, blocks[block_idx], &[]);

            builder.def_var(ret_var, arg1);

            builder.ins().jump(blocks[block_idx], &[]);

            builder.switch_to_block(blocks[block_idx]);
            builder.seal_block(blocks[block_idx]);
        }

        let ret_var = builder.use_var(ret_var);
        builder.ins().return_(&[ret_var]);

        builder.finalize();
    }

    func
}

fn ir_to_jit(func: Function) -> CompiledFn {
    let (set_builder, isa_builder) = builders().unwrap();
    let isa = isa_builder.finish(Flags::new(&set_builder));
    let isa = &*isa;

    // println!("{}", func.display(None));

    let mut context = Context::new();
    context.func = func;

    let len = context.compile(isa).unwrap() as usize;

    let mmap = MemoryMap::new(
        len,
        &[
            MapOption::MapReadable,
            MapOption::MapWritable,
            MapOption::MapExecutable,
        ],
    ).unwrap();

    context.emit_to_memory(mmap.data(), &mut Sink, isa);

    CompiledFn(mmap, len)
}

pub fn jit_compile_part1(instructions: &[Instruction]) -> CompiledFn {
    ir_to_jit(ir_compile_part1(instructions))
}

pub fn ir_compile_part2(instructions: &[Instruction]) -> Function {
    let mut variables = HashSet::new();
    for instruction in instructions {
        variables.insert(instruction.src);
        variables.insert(instruction.dst);
    }
    let variables = variables
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    let ret_var = Variable(variables.len());

    let mut sig = Signature::new(CallConv::Native);
    sig.returns.push(AbiParam::new(I64));
    let mut il_builder = ILBuilder::<Variable>::new();
    let mut func = Function::with_name_signature(ExternalName::user(0, 0), sig);
    {
        let mut builder = FunctionBuilder::<Variable>::new(&mut func, &mut il_builder);

        let blocks = (0..instructions.len() + 1)
            .map(|_| builder.create_ebb())
            .collect::<Vec<_>>();

        for i in 0..variables.len() {
            builder.declare_var(Variable(i), I64);
        }

        builder.declare_var(ret_var, I64);

        let mut block_idx = 0;

        builder.switch_to_block(blocks[block_idx]);
        builder.seal_block(blocks[block_idx]);

        for i in 0..variables.len() {
            let tmp = builder.ins().iconst(I64, 0);
            builder.def_var(Variable(i), tmp);
        }

        let tmp = builder.ins().iconst(I64, 0);
        builder.def_var(ret_var, tmp);

        for instruction in instructions {
            block_idx += 1;

            let arg1 = builder.use_var(Variable(variables[instruction.src]));
            let arg2 = builder.ins().iconst(I64, instruction.cmp_val);
            let cond = match instruction.cmp_op {
                CmpOp::Eq => IntCC::Equal,
                CmpOp::Ne => IntCC::NotEqual,
                CmpOp::Lt => IntCC::SignedLessThan,
                CmpOp::Le => IntCC::SignedLessThanOrEqual,
                CmpOp::Gt => IntCC::SignedGreaterThan,
                CmpOp::Ge => IntCC::SignedGreaterThanOrEqual,
            };

            let tmp = builder.ins().icmp(cond, arg1, arg2);
            builder.ins().brz(tmp, blocks[block_idx], &[]);

            let dst = Variable(variables[instruction.dst]);
            let arg1 = builder.use_var(dst);
            let arg2 = builder.ins().iconst(I64, instruction.val);

            let tmp = if instruction.op == Op::Inc {
                builder.ins().iadd(arg1, arg2)
            } else {
                builder.ins().isub(arg1, arg2)
            };
            builder.def_var(dst, tmp);

            let arg1 = builder.use_var(dst);
            let arg2 = builder.use_var(ret_var);

            let tmp = builder.ins().icmp(IntCC::SignedGreaterThan, arg1, arg2);
            builder.ins().brz(tmp, blocks[block_idx], &[]);

            builder.def_var(ret_var, arg1);

            builder.ins().jump(blocks[block_idx], &[]);

            builder.switch_to_block(blocks[block_idx]);
            builder.seal_block(blocks[block_idx]);
        }

        let ret_var = builder.use_var(ret_var);
        builder.ins().return_(&[ret_var]);

        builder.finalize();
    }

    func
}

pub fn jit_compile_part2(instructions: &[Instruction]) -> CompiledFn {
    ir_to_jit(ir_compile_part2(instructions))
}

pub struct CompiledFn(MemoryMap, usize);

impl CompiledFn {
    pub fn call(&self) -> i64 {
        let fn_ptr: extern "C" fn() -> i64 = unsafe { ::std::mem::transmute(self.0.data()) };
        fn_ptr()
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts(self.0.data(), self.1) }
    }
}

use cretonne::binemit::{CodeOffset, Reloc, RelocSink};

struct Sink;

impl RelocSink for Sink {
    fn reloc_ebb(&mut self, _: CodeOffset, _: Reloc, _: CodeOffset) {}
    fn reloc_external(&mut self, _: CodeOffset, _: Reloc, _: &ExternalName) {}
    fn reloc_jt(&mut self, _: CodeOffset, _: Reloc, _: JumpTable) {}
}
