APP_NAME = BawalPathFinder
BIN_DIR = bin

.PHONY: all build_frontend build_backend run clean stop map map_clean rebuild_all

# Pastikan semua komponen dibangun sebelum menjalankan
all: build_frontend build_backend

# Tambahkan --no-cache supaya C++ lu benar-benar dikompilasi ulang
build_backend:
	@echo "=== Membangun Image Docker ROS 2 Backend (Tanpa Cache) ==="
	docker build --no-cache -t nav2_backend:latest -f Dockerfile .

build_frontend:
	@echo "=== Mengompilasi Frontend Rust (Mode Rilis) ==="
	cd Interface && cargo build --release
	@mkdir -p $(BIN_DIR)
	@cp Interface/target/release/robot_simulation_interface $(BIN_DIR)/$(APP_NAME)
	@echo "=== Eksekusi frontend siap di $(BIN_DIR)/$(APP_NAME) ==="

# Jalankan semua build dulu baru run
run: all
	@echo "=== Meluncurkan Sistem BawalPathFinder ==="
	@bash bash/run_all.sh

# Target untuk reset total jika ada error "planner not found" lagi
rebuild_all: stop clean build_backend build_frontend
	@echo "=== Sistem telah di-rebuild total ==="

stop:
	@echo "=== Menghentikan dan Menghapus Kontainer ==="
	docker rm -f nav2_sim_backend 2>/dev/null || true
	pkill -x $(APP_NAME) || true

clean: stop map_clean
	@echo "=== Menghapus Binary dan Cache ==="
	rm -rf $(BIN_DIR)/$(APP_NAME)
	cd Interface && cargo clean

map:
	@echo "=== Menyiapkan Peta Simulasi ==="
	pip install Pillow faker
	python3 Test/maps/generate_test_map.py
	@echo "=== Sinkronisasi Peta ke ROS Workspace ==="
	@mkdir -p ROS_workspace/src/navigation/maps
	@cp Test/maps/*.yaml ROS_workspace/src/navigation/maps/ 2>/dev/null || true
	@cp Test/maps/*.png ROS_workspace/src/navigation/maps/ 2>/dev/null || true

map_clean:
	@echo "=== Menghapus Aset Peta Lama ==="
	rm -f Test/maps/map_*.yaml Test/maps/map_*.png
	rm -f ROS_workspace/src/navigation/maps/map_*.yaml ROS_workspace/src/navigation/maps/map_*.png