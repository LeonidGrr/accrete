use crate::planet_model::PlanetModel;
use accrete::Planetesimal;

#[derive(Debug, Clone)]
pub enum CoalescenceStatus {
    Created,
    Approaching,
    Coalescing,
    Done,
}

#[derive(Debug, Clone)]
pub struct Coalescence {
    pub source_planet_id: String,
    pub target_planet_id: String,
    pub coalesced_model: PlanetModel,
    pub status: CoalescenceStatus,
}

impl Coalescence {
    pub fn new(
        source_planet_id: &str,
        target_planet_id: &str,
        coalescence_result: Planetesimal,
        time: f64,
    ) -> Self {
        let coalesced_model = PlanetModel::new(coalescence_result, time);
        Coalescence {
            source_planet_id: source_planet_id.to_owned(),
            target_planet_id: target_planet_id.to_owned(),
            coalesced_model,
            status: CoalescenceStatus::Created,
        }
    }
    pub fn update_status(&mut self, planet_models: &mut Vec<PlanetModel>) {
        let Coalescence {
            target_planet_id,
            source_planet_id,
            coalesced_model,
            ..
        } = self;

        let mut source_planet = None;
        let mut target_planet = None;

        for p in planet_models.iter_mut() {
            match &p.planet.id {
                id if id == source_planet_id => source_planet = Some(p),
                id if id == target_planet_id => target_planet = Some(p),
                _ => (),
            }
        }

        match self.status {
            CoalescenceStatus::Created => self.status = CoalescenceStatus::Approaching,
            CoalescenceStatus::Approaching => {
                if let (Some(source_planet), Some(target_planet)) = (source_planet, target_planet) {
                    let current_distance = source_planet.position.distance(target_planet.position);
                    if source_planet.coalescence_a.is_none() {
                        source_planet.coalescence_a = Some(coalesced_model.a);
                        target_planet.coalescence_a = Some(coalesced_model.a);
                    }
                    if current_distance <= 1.0 {
                        self.status = CoalescenceStatus::Coalescing;
                    }
                }
            }
            CoalescenceStatus::Coalescing => {
                if let (Some(_), Some(_)) = (source_planet, target_planet) {
                    planet_models.retain(|p| p.planet.id == *source_planet_id);
                    planet_models.retain(|p| p.planet.id == *target_planet_id);
                    planet_models.push(coalesced_model.clone());
                    self.status = CoalescenceStatus::Done;
                }
            }
            CoalescenceStatus::Done => (),
        };
    }
}
