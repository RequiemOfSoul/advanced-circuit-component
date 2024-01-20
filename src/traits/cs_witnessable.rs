use super::*;

pub trait CSWitnessable<E: Engine> {
    type Witness: Clone + std::fmt::Debug + Eq;
    fn create_witness(&self) -> Option<Self::Witness>;
    fn placeholder_witness() -> Self::Witness;
}

impl<E: Engine> CSWitnessable<E> for () {
    type Witness = ();
    fn create_witness(&self) -> Option<Self::Witness> {
        Some(())
    }
    fn placeholder_witness() -> Self::Witness {
        ()
    }
}

impl<E: Engine> CSWitnessable<E> for Num<E> {
    type Witness = E::Fr;
    fn create_witness(&self) -> Option<Self::Witness> {
        self.get_value()
    }
    fn placeholder_witness() -> Self::Witness {
        E::Fr::zero()
    }
}

impl<E: Engine> CSWitnessable<E> for Boolean {
    type Witness = bool;
    fn create_witness(&self) -> Option<Self::Witness> {
        self.get_value()
    }
    fn placeholder_witness() -> Self::Witness {
        false
    }
}

use franklin_crypto::plonk::circuit::byte::Byte;
use cs_derive::{CSAllocatable, CSVariableLengthEncodable, CSWitnessable};
use crate::scheduler::queues::FullSpongeLikeQueueState;
use crate::vm::vm_state::GlobalContext;

impl<E: Engine> CSWitnessable<E> for Byte<E> {
    type Witness = u8;
    fn create_witness(&self) -> Option<Self::Witness> {
        self.get_byte_value()
    }
    fn placeholder_witness() -> Self::Witness {
        0u8
    }
}

impl<E: Engine, T: CSWitnessable<E>, const N: usize> CSWitnessable<E> for [T; N] {
    type Witness = [T::Witness; N];
    fn create_witness(&self) -> Option<Self::Witness> {
        use std::convert::TryInto;

        let mut tmp = vec![];
        for el in self.iter() {
            let wit = el.create_witness()?;
            tmp.push(wit);
        }

        let result: [T::Witness; N] = tmp.try_into().unwrap();

        Some(result)
    }
    fn placeholder_witness() -> Self::Witness {
        use std::convert::TryInto;

        vec![T::placeholder_witness(); N].try_into().unwrap()
    }
}

impl<E: Engine, T: CSWitnessable<E>> CSWitnessable<E> for Vec<T> {
    type Witness = Vec<T::Witness>;
    fn create_witness(&self) -> Option<Self::Witness> {
        let mut result = vec![];
        for el in self.iter() {
            let wit = el.create_witness()?;
            result.push(wit);
        }

        Some(result)
    }

    fn placeholder_witness() -> Self::Witness {
        vec![]
    }
}