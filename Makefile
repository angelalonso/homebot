all: simulate

.PHONY:build 
run:
	cargo build --features run
	@echo
	@echo Build for Robot Done.

.PHONY:run 
run:
	target/debug/homebot

.PHONY:test
test:
	cargo build --features test
	@echo
	@echo Build for Testing Done.

.PHONY:simulate
simulate: 
	cargo build --features simulate
	@echo
	@echo Build for Simulation Done.
	mkdir -p simulation/controllers/rust_controller/
	cp target/debug/homebot simulation/controllers/rust_controller/rust_controller
	webots simulation/worlds/homebot_simulation_world.wbt

.PHONY:clisimulate
clisimulate: 
	cargo build --features clisimulate
	@echo
	@echo Build for CLI Simulation Done.


.PHONY:clean
clean:
	cargo clean
	rm -f src/bindings.rs
	rm -rf simulation/controllers/rust_controller/
