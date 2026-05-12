#[cfg(test)]
mod tests {
    use crate::events_log::accrete_event::AccreteEvent;
    use crate::events_log::accrete_state::AccreteState;
    use crate::Accrete;

    #[test]
    fn restore_state_default() {
        let mut accrete = Accrete::new(Default::default());
        accrete.events_log = Some(vec![]);
        accrete.post_accretion_intensity = 0;
        let resulting_system = accrete.planetary_system();
        let events = accrete
            .events_log
            .as_ref()
            .expect("default is Some(vec![])");
        let mut accrete_state =
            AccreteState::try_from(&events[0]).expect("Failed to restore Accrete state.");

        for e in events.iter() {
            accrete_state.set_from_event(e);
        }

        assert_eq!(
            format!("{:?}", resulting_system),
            format!("{:?}", accrete_state.system)
        );
    }

    #[test]
    fn events_log_some_by_default() {
        let accrete = Accrete::new(1);
        assert!(
            accrete.events_log.is_some(),
            "default is opt-out: events_log must be Some(vec![])"
        );
        assert_eq!(accrete.events_log.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn events_log_none_opt_out_produces_no_events() {
        let mut accrete = Accrete::new(1);
        accrete.events_log = None;
        accrete.post_accretion_intensity = 5;
        let _ = accrete.planetary_system();
        assert!(
            accrete.events_log.is_none(),
            "None must remain None after planetary_system()"
        );
    }

    #[test]
    fn system_output_identical_with_and_without_log() {
        let mut a1 = Accrete::new(1);
        a1.events_log = None;
        let s1 = a1.planetary_system();

        let mut a2 = Accrete::new(1);
        a2.events_log = Some(vec![]);
        let s2 = a2.planetary_system();

        assert_eq!(
            format!("{:?}", s1),
            format!("{:?}", s2),
            "None and Some(vec![]) paths must produce identical systems"
        );
    }

    #[test]
    fn bombardment_count_matches_intensity() {
        let mut accrete = Accrete::new(1);
        accrete.events_log = Some(vec![]);
        accrete.post_accretion_intensity = 5;
        let _ = accrete.planetary_system();
        let log = accrete.events_log.as_ref().unwrap();
        let injected = log
            .iter()
            .filter(|e| matches!(e, AccreteEvent::OuterBodyInjected(..)))
            .count();
        assert_eq!(
            injected, 5,
            "post_accretion emits one OuterBodyInjected per loop iteration"
        );
    }
}
