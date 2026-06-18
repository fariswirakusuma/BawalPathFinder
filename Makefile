APP_NAME = BawalPathFinder
BIN_DIR = bin

.PHONY: all build_frontend build_backend run clean stop map

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
	rm -rf $(BIN_DIR)/BawalPathFinder
	cd Interface && cargo clean

map:
	@echo "=== Menyiapkan Peta Simulasi ==="
	pip install Pillow faker
	python3 Test/maps/generate_test_map.py
	@echo "=== Sinkronisasi Peta ke ROS Workspace ==="
	@mkdir -p ROS_workspace/src/navigation/maps
	@cp Test/maps/*.yaml ROS_workspace/src/navigation/maps/ 2>/dev/null || true
	@cp Test/maps/*.png ROS_workspace/src/navigation/maps/ 2>/dev/null || true