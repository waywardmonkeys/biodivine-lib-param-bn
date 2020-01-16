use crate::biodivine_std::param::Params;
use biodivine_lib_bdd::Bdd;
use super::BddParams;

impl BddParams {
    /// Consume these `BddParams` and turn them into a raw `Bdd`.
    pub fn into_bdd(self) -> Bdd {
        return self.0;
    }

    pub fn cardinality(&self) -> f64 {
        return self.0.cardinality();
    }
}

impl Params for BddParams {
    fn union(&self, other: &Self) -> Self {
        return BddParams(self.0.or(&other.0));
    }

    fn intersect(&self, other: &Self) -> Self {
        return BddParams(self.0.and(&other.0));
    }

    fn minus(&self, other: &Self) -> Self {
        return BddParams(self.0.and_not(&other.0));
    }

    fn is_empty(&self) -> bool {
        return self.0.is_false();
    }

    fn is_subset(&self, other: &Self) -> bool {
        // TODO: Introduce special function for this in bdd-lib to avoid allocation
        return self.minus(other).is_empty();
    }
}
