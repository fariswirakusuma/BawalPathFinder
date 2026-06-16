APP_NAME = BawalPathFinder
BIN_DIR = bin

.PHONY: all build_frontend build_backend run clean stop

all: build_frontend build_backend

build_frontend:
	@echo "=== Mengompilasi Frontend Rust (Mode Rilis) ==="
	cd Interface && cargo build --release
	@mkdir -p $(BIN_DIR)
	@cp Interface/target/release/robot_simulation_interface $(BIN_DIR)/$(APP_NAME)
	@echo "=== Eksekusi berhasil dibuat pada $(BIN_DIR)/$(APP_NAME) ==="

build_backend:
	@echo "=== Membangun Image Docker ROS 2 Backend ==="
	docker build -t nav2_backend:latest -f Dockerfile .

run: build_frontend
	@echo "=== Meluncurkan Sistem BawalPathFinder ==="
	@bash bash/run_all.sh

stop:
	@echo "=== Menghentikan dan Menghapus Kontainer ==="
	docker rm -f nav2_sim_backend 2>/dev/null || true
	pkill -x $(APP_NAME) || true

clean: stop
	@echo "=== Menghapus Binary dan Cache ==="
	rm -rf $(BIN_DIR)
	cd Interface && cargo clean