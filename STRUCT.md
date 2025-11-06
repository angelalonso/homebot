homebot/
├── Cargo.toml
├── config.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── inputs/
│   │   ├── mod.rs
│   │   ├── input_source.rs
│   │   ├── webots_input.rs
│   │   └── arduino_input.rs
│   ├── actions/
│   │   ├── mod.rs
│   │   ├── action_target.rs
│   │   ├── webots_actions.rs
│   │   └── arduino_actions.rs
│   └── logic/
│       ├── mod.rs
│       ├── motor_control.rs
│       └── pid_controller.rs
└── examples/
    └── webots_simulation.rs
