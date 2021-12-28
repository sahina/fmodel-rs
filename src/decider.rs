trait Command {}

trait Event {}

trait State {}

/// IDecider trait
///
/// # Types
///
///   * `C` - Command type
///   * `Si` - Initial state type
///   * `So` - Output state type
///   * `Ei` - Initial event type
///   * `Eo` - Output event type
trait IDecider<C, Si, So, Ei, Eo>
    where C: Command,
          Si: State,
          So: State,
          Ei: Event,
          Eo: Event
{
    fn decide(&self, command: C, state: Si) -> Vec<Eo>;
    fn evolve(&self, state: Si, event: Ei) -> So;
    fn initial_state(&self) -> &So;
}

/// Decider trait
///
/// # Types
///
///   * `C` - Command type
///   * `S` - State type
///   * `E` - Event type
trait Decider<C, S, E>: IDecider<C, S, S, E, E>
    where C: Command,
          S: State,
          E: Event {
    fn new(
        decide: fn(command: C, state: S) -> Vec<E>,
        evolve: fn(state: S, event: E) -> S,
        initial_state: S,
    ) -> Self;
}

/// Identity trait
trait Identity<T> {
    fn identity(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Number {
        decide_fn: fn(NumberCmd, NumberState) -> Vec<NumberEvt>,
        evolve_fn: fn(NumberState, NumberEvt) -> NumberState,
        initial_state: NumberState,
    }

    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    struct NumberState {
        value: u32,
    }

    impl NumberState {
        fn new(num: u32) -> Self {
            NumberState { value: num }
        }
    }

    impl State for NumberState {}

    enum NumberCmd {
        AddOddNumber(u32),
        MultiplyOddNumber(u32),
        AddEvenNumber(u32),
        MultiplyEvenNumber(u32),
    }

    impl Command for NumberCmd {}

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum NumberEvt {
        OddNumberAdded(u32),
        OddNumberMultiplied(u32),
        EvenNumberAdded(u32),
        EvenNumberMultiplied(u32),
    }

    impl Event for NumberEvt {}

    impl IDecider<NumberCmd, NumberState, NumberState, NumberEvt, NumberEvt> for Number
    {
        fn decide(&self, command: NumberCmd, state: NumberState) -> Vec<NumberEvt> {
            (&self.decide_fn)(command, state)
        }

        fn evolve(&self, state: NumberState, event: NumberEvt) -> NumberState {
            (&self.evolve_fn)(state, event)
        }

        fn initial_state(&self) -> &NumberState {
            &self.initial_state
        }
    }

    impl Decider<NumberCmd, NumberState, NumberEvt> for Number {
        fn new(
            decide: fn(command: NumberCmd, state: NumberState) -> Vec<NumberEvt>,
            evolve: fn(state: NumberState, event: NumberEvt) -> NumberState,
            initial_state: NumberState,
        ) -> Self {
            Number {
                decide_fn: decide,
                evolve_fn: evolve,
                initial_state,
            }
        }
    }

    fn decide(command: NumberCmd, _state: NumberState) -> Vec<NumberEvt> {
        match command {
            NumberCmd::AddEvenNumber(num) => vec![NumberEvt::EvenNumberAdded(num)],
            NumberCmd::AddOddNumber(num) => vec![NumberEvt::OddNumberAdded(num)],
            NumberCmd::MultiplyEvenNumber(num) => vec![NumberEvt::EvenNumberMultiplied(num)],
            NumberCmd::MultiplyOddNumber(num) => vec![NumberEvt::OddNumberMultiplied(num)],
        }
    }

    fn evolve(state: NumberState, event: NumberEvt) -> NumberState {
        match event {
            NumberEvt::OddNumberAdded(num) => NumberState::new(state.value + num),
            NumberEvt::OddNumberMultiplied(num) => NumberState::new(state.value * num),
            NumberEvt::EvenNumberAdded(num) => NumberState::new(state.value + num),
            NumberEvt::EvenNumberMultiplied(num) => NumberState::new(state.value * num),
        }
    }

    #[test]
    fn initial_state() {
        let f = Number::new(decide, evolve, NumberState::new(0));

        let state = f.initial_state().clone();
        let expected = NumberState::new(0);

        assert_eq!(state, expected);
    }

    #[test]
    fn evolve_fn() {
        let f = Number::new(decide, evolve, NumberState::new(0));

        let expected_state = NumberState::new(4);
        let new_state = f.evolve(NumberState::new(2), NumberEvt::EvenNumberAdded(2));

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn decide_fn() {
        let f = Number::new(decide, evolve, NumberState::new(0));

        let events = f.decide(NumberCmd::MultiplyEvenNumber(2), NumberState::new(2));
        let expected = vec![NumberEvt::EvenNumberMultiplied(2)];

        assert_eq!(events, expected)
    }
}


