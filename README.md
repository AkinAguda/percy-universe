# percy-universe

`percy-universe` acts like [react-redux](https://github.com/reduxjs/react-redux) but for [percy](https://github.com/chinedufn/percy) and [app-universe](https://github.com/AkinAguda/app-universe)

### Goals of `percy-universe`:

- Since `percy` does not have any built in state management like react, `percy-universe` aims to be the single solution for state management for percy

- `percy-universe` aims to separate view and logoc at component level. I find that this is just best when building FE applications. Multiple devs can work on the same component with little to no merge conflicts and people who specialize in building UI and people who specialize in writing algorithms can work together on the same front end relatively easily

- Transparancy is also important. What does transparency mean? It means that whenever a component uses state, it's obvious at the point the component is being rendered by a developer that has never seen that component before. This makes it clear what components are smart and what components are dumb simply by using them. This way, a component can safely remain a black box but tells you that "in some way, shape or form, this component uses state"

- Leverage `rust`. This means that we want to lean into the strengths of rust and not try to be some other library like `react-redux`. This crate is **HEAVILY** inspired by `react-redux` but part of the goal might be to come up with patterns that work great with `rust` while learning from libraries like `react-redux`

## Propsed API

### 1. Prop Drilling Solution:

A component's directory would be structured like this:

```
menu
│   menu_view.rs
│   mod.rs
```

- The `menu_view.rs` file will hold strictly **view** code. In this case, **view** code will take data and render a view. This means this component is pure. This file could look something like:

```rust
    struct Menu {
        close_menu: Box<dyn Fn()>
    }

    impl View for Menu {
        fn render() -> VirtualNode {
            html! { /* Some HTML */ }
        }
    }
```

- The The `mod.rs` file will hold all the logic for for taking data from the `universe` and mapping it to the `menu_view.rs` file. This file could look something like:

```rust
    [ /* Some Proc Macro */ ]
    struct Menu {
        universe: AppUniverse<AppState>
    }

    impl View for MenuView {
        fn render(self) -> VirtualNode {
            // This is pseudo code
            let universe_clone = self.universe.clone();

            let close_menu = || {
                universe_clone.msg.send(Msg::CloseMenu)
            }

            html! { <MenuView close_menu={close_menu} /> }
        }
    }
```

- In the event that the menu component does not reach out to any data in the universe, then it will not have a `menu_view.rs` file and will only hold the `mod.rs` file.
- Whenever `Menu` is being used, it will be rendered something like:

```rust
[/* Some Proc Macro */]
<Menu />
```

**Need to find a more efficient way to pass regular data that `MenuView` needs through `Menu`.**
For instance, if `MenuView` needs a color passed in, it will be passed in into `<Menu color="red" />` and `Menu` will pass it into `MenuView`. This is tedious.
