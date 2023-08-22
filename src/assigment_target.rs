use crate::{clr_method::LocalPlacement, rvalue::RValue, Assembly, BaseIR, CLRMethod};

use rustc_middle::mir::Place;
use rustc_middle::mir::ProjectionElem;
pub(crate) enum AsigmentValuePosition {
    BeforeAdress,
    AfterAdress,
}
pub(crate) struct AsigmentTarget {
    adress_calc: Vec<BaseIR>,
    value_pos: AsigmentValuePosition,
    set_ops: Vec<BaseIR>,
}
impl AsigmentTarget {
    pub(crate) fn from_placement(place: Place, clr_method: &CLRMethod, asm: &Assembly) -> Self {
        let mut new = Self {
            adress_calc: Vec::new(),
            set_ops: Vec::new(),
            value_pos: AsigmentValuePosition::BeforeAdress,
        };
        if place.projection.is_empty() {
            let local: u32 = place.local.into();
            new.set_ops
                .push(match clr_method.local_id_placement(local) {
                    LocalPlacement::Arg(arg_id) => BaseIR::STArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::STLoc(var_id),
                });
        } else {
            let local: u32 = place.local.into();
            new.adress_calc
                .push(match clr_method.local_id_placement(local) {
                    LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
                    LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
                });
            new.value_pos = AsigmentValuePosition::AfterAdress;
            let (adress_calc, set_op) = crate::projection::projection_set(
                &place.projection,
                clr_method.get_type_of_local(place.local.into()),
                clr_method,
                asm,
            );
            //let local: u32 = place.local.into();
            new.adress_calc.extend(adress_calc);
            new.set_ops.push(set_op);
        }
        new
    }
    pub(crate) fn finalize_with_ops(self, ops: &[BaseIR], clr_method: &mut CLRMethod) {
        match self.value_pos {
            AsigmentValuePosition::BeforeAdress => {
                clr_method.extend_ops(ops);
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(&self.set_ops);
            }
            AsigmentValuePosition::AfterAdress => {
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(ops);
                clr_method.extend_ops(&self.set_ops);
            }
        }
    }
    pub(crate) fn finalize(self, rvalue: RValue, clr_method: &mut CLRMethod) {
        match self.value_pos {
            AsigmentValuePosition::BeforeAdress => {
                clr_method.extend_ops(rvalue.get_ops());
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(&self.set_ops);
            }
            AsigmentValuePosition::AfterAdress => {
                clr_method.extend_ops(&self.adress_calc);
                clr_method.extend_ops(rvalue.get_ops());
                clr_method.extend_ops(&self.set_ops);
            }
        }
    }
}
