//! A generic stack-based state machine.
//! This state machine contains a stack of states and handles transitions between them.
//! StateTransition happen based on the return value of the currently running state's functions.
//! Only one state can run at once.
#![deny(missing_docs)]

use std::marker::PhantomData;

/// A transition from one state to the other.
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
pub enum StateTransition<S: Clone> {
    /// Stay in the current state.
    None,
    /// End the current state and go to the previous state on the stack, if any.
    /// If we Pop the last state, the state machine exits.
    Pop,
    /// Push a new state on the stack.
    Push(S),
    /// Pop all states on the stack and insert this one.
    Switch(S),
    /// Pop all states and exit the state machine.
    Quit,
}

/// Trait that states must implement.
///
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
pub trait Handler<S: Clone, D> {
    /// Called when the state is first inserted on the stack.
    fn on_start(&self, _state: &S, _state_data: &mut D) {}
    /// Called when the state is popped from the stack.
    fn on_stop(&self, _state: &S, _state_data: &mut D) {}
    /// Called when a state is pushed over this one in the stack.
    fn on_pause(&self, _state: &S, _state_data: &mut D) {}
    /// Called when the state just on top of this one in the stack is popped.
    fn on_resume(&self, _state: &S, _state_data: &mut D) {}
    /// Executed on every frame immediately, as fast as the engine will allow.
    /// If you need to execute logic at a predictable interval (for example, a physics engine)
    /// it is suggested to use the state data information to determine when to run such fixed timed
    /// logic.
    fn update(&self, _state: &S, _state_data: &mut D) -> StateTransition<S> {
        StateTransition::None
    }
}

pub struct StateStack<S: Clone> {
    state_stack: Vec<S>,
}

impl<S: Clone> StateStack<S> {
    pub fn new() -> Self {
        StateStack {
            state_stack: vec![],
        }
    }

    pub fn new_initial_state(initial_state: S) -> Self {
        StateStack {
            state_stack: vec![initial_state],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.state_stack.is_empty()
    }

    pub fn last(&self) -> Option<&S> {
        self.state_stack.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut S> {
        self.state_stack.last_mut()
    }

    pub fn pop(&mut self) -> Option<S> {
        self.state_stack.pop()
    }

    pub fn push(&mut self, s: S) {
        self.state_stack.push(s);
    }
}

/// A state machine that holds the stack of states and performs transitions between states.
/// It can be created using
/// ```rust,ignore
/// StateMachine::<()>::default()
/// ```
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
pub struct StateMachine;

impl StateMachine {
    /// Returns if the state machine still has states in its stack.
    pub fn is_running<S: Clone>(state_stack: &StateStack<S>) -> bool {
        !state_stack.is_empty()
    }

    /// Updates the state at the top of the stack with the provided data.
    /// If the states returns a transition, perform it.
    pub fn update<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        let trans = match state_stack.last_mut() {
            Some(state) => handler.update(state, state_data),
            None => StateTransition::None,
        };

        Self::transition(handler, trans, state_stack, state_data);
    }

    fn transition<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        request: StateTransition<S>,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        match request {
            StateTransition::None => (),
            StateTransition::Pop => Self::pop(handler, state_stack, state_data),
            StateTransition::Push(state) => Self::push(handler, state, state_stack, state_data),
            StateTransition::Switch(state) => Self::switch(handler, state, state_stack, state_data),
            StateTransition::Quit => Self::stop(handler, state_stack, state_data),
        }
    }

    fn switch<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        state: S,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        if let Some(state) = state_stack.pop() {
            handler.on_stop(&state, state_data)
        }

        handler.on_start(&state, state_data);
        state_stack.push(state);
    }

    /// Push a state on the stack and start it.
    /// Pauses any previously active state.
    pub fn push<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        state: S,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        if let Some(state) = state_stack.last_mut() {
            handler.on_pause(&state, state_data);
        }

        handler.on_start(&state, state_data);
        state_stack.push(state);
    }

    fn pop<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        if let Some(state) = state_stack.pop() {
            handler.on_stop(&state, state_data);
        }

        if let Some(state) = state_stack.last() {
            handler.on_resume(state, state_data);
        }
    }

    /// Removes all currently running states from the stack.
    pub fn stop<S: Clone, D, H: Handler<S, D>>(
        handler: &H,
        state_stack: &mut StateStack<S>,
        state_data: &mut D,
    ) {
        while let Some(state) = state_stack.pop() {
            handler.on_stop(&state, state_data);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum State {
        A,
        B,
    }

    type StateData<'a> = (&'a mut isize, isize);

    pub struct Test;

    impl<'a> Handler<State, StateData<'a>> for Test {
        fn on_start(&self, state: &State, data: &mut StateData) {
            *data.0 += data.1;
        }

        fn on_resume(&self, state: &State, data: &mut StateData) {
            self.on_start(state, data);
        }

        fn update(&self, _state: &State, _data: &mut StateData) -> StateTransition<State> {
            StateTransition::Push(State::B)
        }
    }

    #[test]
    fn sm_test() {
        let mut sm = StateMachine;

        let mut state_stack = StateStack::new();
        let mut state_data = (0, 10);
        let foo = &mut (&mut state_data.0, state_data.1);

        StateMachine::push(&Test, State::A, &mut state_stack, foo);
        assert!(*foo.0 == 10);

        StateMachine::update(&Test, &mut state_stack, foo);
        assert!(*foo.0 == 20);

        StateMachine::stop(&Test, &mut state_stack, foo);
        assert!(*foo.0 == 20);
        assert!(!StateMachine::is_running(&state_stack))
    }
}
