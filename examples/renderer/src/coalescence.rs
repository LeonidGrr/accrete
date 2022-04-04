use crate::{consts::COALESCE_DISTANCE, planet_model::PlanetModel, state::PlanetModels};
use accrete::Planetesimal;

#[derive(Debug, Clone)]
enum CoalescenceStatus {
    Created,
    Approaching,
    Coalescing,
    Done,
}

#[derive(Debug, Clone)]
struct Coalescence {
    source_planet_id: String,
    target_planet_id: String,
    coalesced_model: PlanetModel,
    status: CoalescenceStatus,
}

pub struct CoalescenceOption(Option<Coalescence>);

impl CoalescenceOption {
    pub fn none() -> Self {
        CoalescenceOption(None)
    }
    pub fn new(
        source_planet_id: &str,
        target_planet_id: &str,
        coalescence_result: Planetesimal,
        time: f64,
    ) -> Self {
        let coalesced_model = PlanetModel::new(coalescence_result, time);
        let c = Coalescence {
            source_planet_id: source_planet_id.to_owned(),
            target_planet_id: target_planet_id.to_owned(),
            coalesced_model,
            status: CoalescenceStatus::Created,
        };
        CoalescenceOption(Some(c))
    }
    pub fn update_status(&mut self, planet_models: &mut PlanetModels, event_lock: &mut bool) {
        if let Some(c) = &mut self.0 {
            let Coalescence {
                target_planet_id,
                source_planet_id,
                coalesced_model,
                ..
            } = c;

            let mut source_planet = None;
            let mut target_planet = None;

            for p in planet_models.values_mut() {
                match &p.planet.id {
                    id if id == source_planet_id => source_planet = Some(p),
                    id if id == target_planet_id => target_planet = Some(p),
                    _ => (),
                }
            }

            match c.status {
                CoalescenceStatus::Created => {
                    *event_lock = true;
                    c.status = CoalescenceStatus::Approaching
                }
                CoalescenceStatus::Approaching => {
                    if let (Some(source_planet), Some(target_planet)) =
                        (source_planet, target_planet)
                    {
                        if source_planet.target_a.is_none() {
                            source_planet.target_a = Some(coalesced_model.a);
                            target_planet.target_a = Some(coalesced_model.a);
                        }

                        let distance = source_planet.position.distance(target_planet.position);
                        println!("Distance to coalesce: {} au", distance);
                        if distance <= COALESCE_DISTANCE {
                            c.status = CoalescenceStatus::Coalescing;
                        }
                    }
                }
                CoalescenceStatus::Coalescing => {
                    if source_planet.is_some() && target_planet.is_some() {
                        planet_models.remove(source_planet_id);
                        planet_models.remove(target_planet_id);
                        planet_models.insert(coalesced_model.id.clone(), coalesced_model.clone());
                        c.status = CoalescenceStatus::Done;
                    }
                }
                CoalescenceStatus::Done => {
                    *self = CoalescenceOption(None);
                    *event_lock = false;
                }
            };
        }
    }
}
