use super::{super::lower::IntoPolymorphic, encoder::encode_builtin_function_name_with_type_args};
use crate::encoder::{
    builtin_encoder::{BuiltinEncoder, BuiltinFunctionKind},
    encoder::SubstMap,
    errors::{EncodingError, EncodingResult, SpannedEncodingResult, WithSpan},
    high::builtin_functions::encoder::encode_builtin_function_def,
    mir::pure::PureFunctionEncoderInterface,
    snapshot::interface::SnapshotEncoderInterface,
    stub_function_encoder::StubFunctionEncoder,
};
use log::{debug, trace};
use prusti_interface::{data::ProcedureDefId, environment::Environment};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use rustc_middle::ty::TyCtxt;
use std::cell::{Ref, RefCell};
use vir_crate::{
    high::{self as vir_high, operations::ty::Typed},
    polymorphic as vir_poly,
};

#[derive(Default)]
pub(crate) struct HighBuiltinFunctionEncoderState {
    // FIXME: This should be a HashMap into FunctionIdentifier.
    builtin_functions_high: RefCell<HashSet<BuiltinFunctionHighKind>>,
    // FIXME: This should be removed once BuiltinEncoder is fully migrated to high.
    builtin_functions: RefCell<HashMap<BuiltinFunctionKind, vir_poly::FunctionIdentifier>>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum BuiltinFunctionHighKind {
    /// type
    Unreachable(vir_high::Type),
    /// type
    Undefined(vir_high::Type),
    #[allow(dead_code)] // Will be used once fully migrated from BuiltinFunctionKind
    /// array lookup pure function
    ArrayLookupPure {
        array_pred_type: vir_high::Type,
        elem_pred_type: vir_high::Type,
        array_len: usize,
        return_ty: vir_high::Type,
    },
    #[allow(dead_code)] // Will be used once fully migrated from BuiltinFunctionKind
    /// lookup_pure function for slices
    SliceLookupPure {
        slice_pred_type: vir_high::Type,
        elem_pred_type: vir_high::Type,
        return_ty: vir_high::Type,
    },
    /// abstract length function for slices
    SliceLen {
        slice_pred_type: vir_high::Type,
        elem_pred_type: vir_high::Type,
    },
}

impl IntoPolymorphic<BuiltinFunctionKind> for BuiltinFunctionHighKind {
    fn lower(
        &self,
        encoder: &impl crate::encoder::high::types::interface::HighTypeEncoderInterfacePrivate,
    ) -> BuiltinFunctionKind {
        match self {
            BuiltinFunctionHighKind::Unreachable(ty) => {
                BuiltinFunctionKind::Unreachable(ty.lower(encoder))
            }
            BuiltinFunctionHighKind::Undefined(ty) => {
                BuiltinFunctionKind::Undefined(ty.lower(encoder))
            }
            BuiltinFunctionHighKind::ArrayLookupPure {
                array_pred_type,
                elem_pred_type,
                array_len,
                return_ty,
            } => BuiltinFunctionKind::ArrayLookupPure {
                array_pred_type: array_pred_type.lower(encoder),
                elem_pred_type: elem_pred_type.lower(encoder),
                array_len: *array_len,
                return_ty: return_ty.lower(encoder),
            },
            BuiltinFunctionHighKind::SliceLookupPure {
                slice_pred_type,
                elem_pred_type,
                return_ty,
            } => BuiltinFunctionKind::SliceLookupPure {
                slice_pred_type: slice_pred_type.lower(encoder),
                elem_pred_type: elem_pred_type.lower(encoder),
                return_ty: return_ty.lower(encoder),
            },
            BuiltinFunctionHighKind::SliceLen {
                slice_pred_type,
                elem_pred_type,
            } => BuiltinFunctionKind::SliceLen {
                slice_pred_type: slice_pred_type.lower(encoder),
                elem_pred_type: elem_pred_type.lower(encoder),
            },
        }
    }
}

trait HighBuiltinFunctionEncoderInterfacePrivate<'tcx> {
    fn encode_builtin_function_def_high(&self, function_kind: BuiltinFunctionHighKind);
    fn encode_builtin_function_name_with_type_args_high(
        &self,
        function: &BuiltinFunctionHighKind,
    ) -> (String, Vec<vir_high::Type>);
    fn encode_builtin_function_def(&self, function_kind: BuiltinFunctionKind);
}

impl<'v, 'tcx: 'v> HighBuiltinFunctionEncoderInterfacePrivate<'tcx>
    for crate::encoder::encoder::Encoder<'v, 'tcx>
{
    fn encode_builtin_function_def_high(&self, function_kind: BuiltinFunctionHighKind) {
        trace!("encode_builtin_function_def_high({:?})", function_kind);
        if !self
            .high_builtin_function_encoder_state
            .builtin_functions_high
            .borrow()
            .contains(&function_kind)
        {
            let _function = encode_builtin_function_def(function_kind.clone()); // TODO
            self.high_builtin_function_encoder_state
                .builtin_functions_high
                .borrow_mut()
                .insert(function_kind);
        }
    }
    fn encode_builtin_function_name_with_type_args_high(
        &self,
        function: &BuiltinFunctionHighKind,
    ) -> (String, Vec<vir_high::Type>) {
        encode_builtin_function_name_with_type_args(function)
    }
    fn encode_builtin_function_def(&self, function_kind: BuiltinFunctionKind) {
        trace!("encode_builtin_function_def({:?})", function_kind);
        if !self
            .high_builtin_function_encoder_state
            .builtin_functions
            .borrow()
            .contains_key(&function_kind)
        {
            let builtin_encoder = BuiltinEncoder::new(self);
            let function = builtin_encoder.encode_builtin_function_def(function_kind.clone());
            self.high_builtin_function_encoder_state
                .builtin_functions
                .borrow_mut()
                .insert(function_kind, self.insert_function(function));
        }
    }
}

pub(crate) trait HighBuiltinFunctionEncoderInterface<'tcx> {
    fn encode_builtin_function_use_high(
        &self,
        function_kind: BuiltinFunctionHighKind,
    ) -> (String, Vec<vir_high::Type>);
    fn encode_builtin_function_use(
        &self,
        function_kind: BuiltinFunctionKind,
    ) -> (String, Vec<vir_poly::Type>);
    // FIXME: Made public only to be accessible from old BuiltinEncoder.
    fn encode_builtin_function_name_with_type_args(
        &self,
        function: &BuiltinFunctionKind,
    ) -> (String, Vec<vir_poly::Type>);
}

impl<'v, 'tcx: 'v> HighBuiltinFunctionEncoderInterface<'tcx>
    for crate::encoder::encoder::Encoder<'v, 'tcx>
{
    fn encode_builtin_function_use_high(
        &self,
        function_kind: BuiltinFunctionHighKind,
    ) -> (String, Vec<vir_high::Type>) {
        trace!("encode_builtin_function_use_high({:?})", function_kind);
        if !self
            .high_builtin_function_encoder_state
            .builtin_functions_high
            .borrow()
            .contains(&function_kind)
        {
            // Trigger encoding of definition
            self.encode_builtin_function_def_high(function_kind.clone());
        }
        self.encode_builtin_function_name_with_type_args_high(&function_kind)
    }
    fn encode_builtin_function_use(
        &self,
        function_kind: BuiltinFunctionKind,
    ) -> (String, Vec<vir_poly::Type>) {
        trace!("encode_builtin_function_use({:?})", function_kind);
        // FIXME: We should use encode_builtin_function_use_high with lowering instead..
        if !self
            .high_builtin_function_encoder_state
            .builtin_functions
            .borrow()
            .contains_key(&function_kind)
        {
            self.encode_builtin_function_def(function_kind.clone());
        }
        self.encode_builtin_function_name_with_type_args(&function_kind)
    }
    fn encode_builtin_function_name_with_type_args(
        &self,
        kind: &BuiltinFunctionKind,
    ) -> (String, Vec<vir_poly::Type>) {
        // FIXME: This should use
        // super::encoder::encode_builtin_function_name_with_type_args and do
        // lowering once we get rid of the old BuiltinEncoder.
        match kind {
            BuiltinFunctionKind::Unreachable(typ) => {
                ("builtin$unreach".to_string(), vec![typ.clone()])
            }
            BuiltinFunctionKind::Undefined(typ) => ("builtin$undef".to_string(), vec![typ.clone()]),
            BuiltinFunctionKind::ArrayLookupPure {
                array_pred_type: container_type,
                elem_pred_type,
                ..
            }
            | BuiltinFunctionKind::SliceLookupPure {
                slice_pred_type: container_type,
                elem_pred_type,
                ..
            } => (
                "lookup_pure".to_string(),
                vec![container_type.clone(), elem_pred_type.clone()],
            ),
            BuiltinFunctionKind::SliceLen { elem_pred_type, .. } => {
                ("Slice$len".to_string(), vec![elem_pred_type.clone()])
            }
        }
    }
}
