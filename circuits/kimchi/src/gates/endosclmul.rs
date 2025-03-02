/*****************************************************************************************************************

This source file implements group endomorphism optimised
variable base scalar multiplication custom Plonk constraints.

EVBSM gate constraints
    b1*(b1-1) = 0
    b2*(b2-1) = 0
    b3*(b3-1) = 0
    b4*(b4-1) = 0
    ((1 + (endo - 1) * b2) * xt - xp) * s1 = (2*b1-1)*yt - yp
    (2*xp – s1^2 + (1 + (endo - 1) * b2) * xt) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
    (yr + yp)^2 = (xp – xr)^2 * (s1^2 – (1 + (endo - 1) * b2) * xt + xr)
    ((1 + (endo - 1) * b2) * xt - xr) * s3 = (2*b3-1)*yt - yr
    (2*xr – s3^2 + (1 + (endo - 1) * b4) * xt) * ((xr – xs) * s3 + ys + yr) = (xr – xs) * 2*yr
    (ys + yr)^2 = (xr – xs)^2 * (s3^2 – (1 + (endo - 1) * b4) * xt + xs)
    n_next = 16*n + 8*b1 + 4*b2 + 2*b3 + b4

The constraints above are derived from the following EC Affine arithmetic equations:

    (xq1 - xp) * s1 = yq1 - yp
    (2*xp – s1^2 + xq1) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
    (yr + yp)^2 = (xp – xr)^2 * (s1^2 – xq1 + xr)

    (xq2 - xr) * s3 = yq2 - yr
    (2*xr – s3^2 + xq2) * ((xr – xs) * s3 + ys + yr) = (xr – xs) * 2*yr
    (ys + yr)^2 = (xr – xs)^2 * (s3^2 – xq2 + xs)

*****************************************************************************************************************/

use crate::gate::{CircuitGate, GateType};
use crate::{
    nolookup::constraints::ConstraintSystem,
    wires::{GateWires, COLUMNS},
};
use ark_ff::FftField;
use array_init::array_init;

impl<F: FftField> CircuitGate<F> {
    pub fn create_endomul(wires: GateWires) -> Self {
        CircuitGate {
            typ: GateType::EndoMul,
            wires,
            c: vec![],
        }
    }

    pub fn verify_endomul(
        &self,
        row: usize,
        witness: &[Vec<F>; COLUMNS],
        cs: &ConstraintSystem<F>,
    ) -> Result<(), String> {
        let this: [F; COLUMNS] = array_init(|i| witness[i][row]);
        let next: [F; COLUMNS] = array_init(|i| witness[i][row + 1]);
        let xq1 = (F::one() + ((cs.endo - F::one()) * next[12])) * this[0];
        let xq2 = (F::one() + ((cs.endo - F::one()) * next[14])) * this[0];

        ensure_eq!(self.typ, GateType::EndoMul, "incorrect gate type");

        // verify booleanity of the scalar bits

        ensure_eq!(F::zero(), this[11] - this[11].square(), "wrong eq 1");
        ensure_eq!(F::zero(), this[12] - this[12].square(), "wrong eq 2");
        ensure_eq!(F::zero(), this[13] - this[13].square(), "wrong eq 3");
        ensure_eq!(F::zero(), this[14] - this[14].square(), "wrong eq 4");
        ensure_eq!(
            F::zero(),
            (xq1 - this[4]) * this[9] - (this[11].double() - F::one()) * this[2] + this[5],
            "wrong eq 5"
        );
        ensure_eq!(
            F::zero(),
            (this[4].double() - this[9].square() + xq1)
                * ((this[4] - this[7]) * this[9] + this[8] + this[5])
                - (this[4] - this[7]) * this[5].double(),
            "wrong eq 6"
        );
        ensure_eq!(
            F::zero(),
            (this[8] + this[5]).square()
                - (this[4] - this[7]).square() * (this[9].square() - xq1 + this[7]),
            "wrong eq 7"
        );
        ensure_eq!(
            F::zero(),
            (xq2 - this[7]) * this[10] - (this[13].double() - F::one()) * this[2] + this[8],
            "wrong eq 8"
        );
        ensure_eq!(
            F::zero(),
            (this[7].double() - this[10].square() + xq2)
                * ((this[7] - this[2]) * this[10] + this[3] + this[8])
                - (this[7] - this[2]) * this[8].double(),
            "wrong eq 9"
        );
        ensure_eq!(
            F::zero(),
            (this[3] + this[8]).square()
                - (this[7] - this[2]).square() * (this[10].square() - xq2 + this[2]),
            "wrong eq 10"
        );
        ensure_eq!(
            F::zero(),
            (((witness[6][row + 1].double() + this[11]).double() + this[12]).double() + this[13])
                .double()
                + this[14]
                - this[6],
            "wrong eq 11"
        );

        Ok(())
    }

    pub fn endomul(&self) -> F {
        if self.typ == GateType::EndoMul {
            F::one()
        } else {
            F::zero()
        }
    }
}
