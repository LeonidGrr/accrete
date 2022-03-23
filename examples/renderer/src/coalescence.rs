use accrete::Planetesimal;

use crate::planet_model::PlanetModel;

#[derive(Debug, Clone)]
pub enum CoalescenceStatus {
    Approaching,
    Coalescing,
    Done,
}

#[derive(Debug, Clone)]
 pub struct Coalescence {
    pub source_planet_id: String,
    pub target_planet_id: String,
    pub coalescence_result: Planetesimal,
    pub coalesced_model: PlanetModel,
    pub status: CoalescenceStatus
}

impl Coalescence {
    pub fn new(source_planet_id: String, target_planet_id: String, coalescence_result: Planetesimal, dt: f32) -> Self {
        let coalesced_model = PlanetModel::new(&coalescence_result, dt);
        Coalescence {
            source_planet_id,
            target_planet_id,
            coalescence_result,
            coalesced_model,
            status: CoalescenceStatus::Approaching,
        }
    }
}