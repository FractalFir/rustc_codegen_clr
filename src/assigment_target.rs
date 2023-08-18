use crate::rvalue::RValue;
use crate::{clr_method::LocalPlacement, BaseIR, CLRMethod};

use rustc_middle::mir::Place;
use rustc_middle::mir::ProjectionElem;
enum AsigmentValuePosition {
    BeforeAdress,
    AfterAdress,
}
pub(crate) struct AsigmentTarget {
    adress_calc: Vec<BaseIR>,
    value_pos: AsigmentValuePosition,
    set_ops: Vec<BaseIR>,
}
impl AsigmentTarget {
    pub(crate) fn from_placement(place: Place, clr_method: &CLRMethod) -> Self {
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
            for _modifier in &place.projection[..(place.projection.len() - 1)] {
                todo!("Can't handle assignments with more than one level of indirection")
            }
            let last = place.projection[place.projection.len() - 1];
            match last {
                ProjectionElem::Deref => {
                    //TODO: handle the type
                    new.value_pos = AsigmentValuePosition::AfterAdress;
                    new.set_ops.push(BaseIR::STIInd(4));
                }
                _ => todo!("Can't handle ProjectionElements of type {last:?}!"),
            }
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
