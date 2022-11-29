# percy-universe

`percy-universe` acts like [react-redux](https://github.com/reduxjs/react-redux) but for [percy](https://github.com/chinedufn/percy) and [app-universe](https://github.com/AkinAguda/app-universe)

### Goals of `percy-universe`:

- Since `percy` does not have any built in state management like react, `percy-universe` aims to be the single solution for state management for percy

- `percy-universe` aims to separate view and logoc at component level. I find that this is just best when building FE applications. Multiple devs can work on the same component with little to no merge conflicts and people who specialize in building UI and people who specialize in writing algorithms can work together on the same front end relatively easily

- Transparancy is also important. What does transparency mean? It means that whenever a component uses state, it's obvious at the point the component is being rendered by a developer that has never seen that component before. This makes it clear what components are smart and what components are dumb simply by using them. This way, a component can safely remain a black box but tells you that "in some way, shape or form, this component uses state"

- Leverage `rust`. This means that we want to lean into the strengths of rust and not try to be some other library like `react-redux`. This crate is **HEAVILY** inspired by `react-redux` but part of the goal might be to come up with patterns that work great with `rust` while learning from libraries like `react-redux`
