use accrete::Planetesimal;

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
    pub status: CoalescenceStatus
}

impl Coalescence {
    pub fn new(source_planet_id: String, target_planet_id: String, coalescence_result: Planetesimal) -> Self {
        Coalescence {
            source_planet_id,
            target_planet_id,
            coalescence_result,
            status: CoalescenceStatus::Approaching,
        }
    }
}