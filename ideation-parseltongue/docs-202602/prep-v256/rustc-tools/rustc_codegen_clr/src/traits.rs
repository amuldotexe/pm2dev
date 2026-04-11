#![allow(unused_variables)]
use std::{path::PathBuf, sync::Arc};

use crate::{builder::{Builder, CodegenCx}, CillyBackend};
use cilly::Assembly;
use rustc_ast::expand::{allocator::AllocatorKind, autodiff_attrs::AutoDiffItem};
use rustc_codegen_ssa::{
    back::{
        lto::{SerializedModule, ThinModule},
        write::*,
    },
    mono_item::MonoItemExt,
    traits::*,
    *,
};
use rustc_errors::*;
use rustc_middle::{dep_graph::*, ty::*};
use rustc_session::{config::*, *};
use rustc_span::{fatal_error::FatalError, Symbol};
pub struct Module {
    asm: Assembly,
}
pub struct ModuleBuffer {
    data: Vec<u8>,
}
pub struct ThinBuffer {
    data: Vec<u8>,
    thin_link_data: Vec<u8>,
}
impl ModuleBufferMethods for ModuleBuffer {
    fn data(&self) -> &[u8] {
        &self.data
    }
}
impl ThinBufferMethods for ThinBuffer {
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn thin_link_data(&self) -> &[u8] {
        &self.thin_link_data
    }
}
impl ExtraBackendMethods for CillyBackend{
    fn codegen_allocator<'tcx>(
        &self,
        tcx: TyCtxt<'tcx>,
        module_name: &str,
        kind: AllocatorKind,
        alloc_error_handler_kind: AllocatorKind,
    ) -> Self::Module {
        todo!()
    }
    fn compile_codegen_unit(
        &self,
        tcx: TyCtxt<'_>,
        cgu_name: Symbol,
    ) -> (ModuleCodegen<Self::Module>, u64) {
        let mut codegen_cx = CodegenCx::new(tcx);
        let cgu = tcx.codegen_unit(cgu_name);
        let mono_items = cgu.items_in_deterministic_order(tcx);
        for &(mono_item, data) in &mono_items {
            mono_item.predefine::<Builder>(
                &mut codegen_cx,
                cgu_name.as_str(),
                data.linkage,
                data.visibility,
            );
        }

        // ... and now that we have everything pre-defined, fill out those definitions.
        for &(mono_item, item_data) in &mono_items {
            mono_item.define::<Builder>(&mut codegen_cx, cgu_name.as_str(), item_data);
        }
        todo!()
    }
    fn target_machine_factory(
        &self,
        sess: &Session,
        opt_level: OptLevel,
        target_features: &[String],
    ) -> TargetMachineFactoryFn<Self> {
        Arc::new(|_| Ok(()))
    }
}
impl WriteBackendMethods for CillyBackend {
    type Module = Module;
    type TargetMachine = ();
    type TargetMachineError = ();
    type ModuleBuffer = ModuleBuffer;
    type ThinData = ();
    type ThinBuffer = ThinBuffer;
    fn run_and_optimize_fat_lto(
        cgcx: &CodegenContext<Self>,
        exported_symbols_for_lto: &[String],
        each_linked_rlib_for_lto: &[PathBuf],
        modules: Vec<FatLtoInput<Self>>,
        diff_fncs: Vec<AutoDiffItem>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }
    fn run_thin_lto(
        cgcx: &CodegenContext<Self>,
        exported_symbols_for_lto: &[String],
        each_linked_rlib_for_lto: &[PathBuf],
        modules: Vec<(String, Self::ThinBuffer)>,
        cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<(Vec<ThinModule<Self>>, Vec<WorkProduct>), FatalError> {
        todo!()
    }
    fn print_pass_timings(&self) {
        todo!()
    }
    fn print_statistics(&self) {
        todo!()
    }
    fn optimize(
        cgcx: &CodegenContext<Self>,
        dcx: DiagCtxtHandle<'_>,
        module: &mut ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
    ) -> Result<(), FatalError> {
        todo!()
    }
    fn optimize_thin(
        cgcx: &CodegenContext<Self>,
        thin: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }
    fn codegen(
        cgcx: &CodegenContext<Self>,
        module: ModuleCodegen<Self::Module>,
        config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        todo!()
    }
    fn prepare_thin(
        module: ModuleCodegen<Self::Module>,
        want_summary: bool,
    ) -> (String, Self::ThinBuffer) {
        todo!()
    }
    fn serialize_module(module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        todo!()
    }
}
