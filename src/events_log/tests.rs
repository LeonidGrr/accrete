#[cfg(test)]
mod tests {
    use crate::Accrete;
    use crate::events_log::accrete_state::AccreteState;

    #[test]
    fn restore_state_default() {
        let mut accrete = Accrete::new(Default::default());
        accrete.post_accretion_intensity = 0;
        let resulting_system = accrete.planetary_system();
        let mut accrete_state = AccreteState::try_from(&accrete.events_log[0]).expect("Failed to restore Accrete state.");

        for e in accrete.events_log.iter() {
            accrete_state.set_from_event(e);
        }

        assert_eq!(format!("{:?}", resulting_system), format!("{:?}", accrete_state.system));
    }
}
