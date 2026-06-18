# BawalPathFinder 🐟

![Version](https://img.shields.io/badge/Release-v0.1.0-blue)
![Build](https://img.shields.io/badge/Build-Makefile_Ready-success)
![Language](https://img.shields.io/badge/Language-C++17-orange)

[English](README_en.md) | [Indonesia](README.md)

Hey there! Welcome to **BawalPathFinder**. Basically, this is a robotics simulation system for pathfinding. For the backend, we're using **ROS 2 Humble (Nav2)** running safely inside Docker. As for the visuals (frontend), we're rocking the **Bevy Engine (Rust)** running natively on your OS to show off those sweet 2D/3D graphics.

## System Requirements (To Keep Things Smooth)

To make sure everything runs without a hitch, make sure you have these ready:
- **OS**: Linux (Highly recommended!)
- **Rust / Bevy**: Cargo & Rustc (Edition 2021), and Bevy Engine v0.18
- **C++**: Gotta be C++17 (Absolute must for ROS 2 Humble)
- **Docker**: Mandatory for running the backend inside a container.
- **Python 3**: Needed to generate testing maps (requires Pillow and faker libraries).

## About the Docker Image

To keep the backend environment clean and isolated, we wrapped it up in Docker.
- **Base Image**: `osrf/ros:humble-desktop`
- **Output Image**: `nav2_backend:latest`

## Setup & How to Run

The easiest and fastest way is using `make`. Here's the drill:

```bash
# 1. Generate and prep the simulation maps first
make map

# 2. Run the whole thing (it auto-builds Frontend & Backend)
make run
```

Or, if you prefer running bash scripts manually, you can do:

```bash
# Give execution permission to all bash scripts first
chmod +x bash/*.sh

# Then just run the main simulation
./bash/run_all.sh
```

## Full Command List

### Makefile Commands

If you're on team `make`, here's the list:

| Command | What does it do? |
| --- | --- |
| `make all` | Builds both the frontend (Rust) and the backend image (Docker). |
| `make build_frontend`| Compiles the Rust UI in release mode (`--release`) into the `bin/` folder. |
| `make build_backend` | Creates the `nav2_backend:latest` Docker image from the `Dockerfile`. |
| `make run` | Runs the entire system (spins up the UI frontend and the backend container). |
| `make stop` | Stops the ROS container and kills the frontend process. |
| `make clean` | Wipes the frontend binaries, Cargo cache, and deletes the maps. |
| `make map` | Generates synthetic testing maps using Python and moves them to the ROS workspace. |
| `make map_clean` | Deletes all map files from your computer and the ROS workspace. |

### Bash Scripts (`bash/`)

If you'd rather hang out in the `bash/` folder:

| Script | What does it do? |
| --- | --- |
| `run_all.sh` | Cleans up old containers, builds a new docker image, runs the backend, and launches the Bevy UI. The whole package. |
| `run_backend.sh` | Cleans & rebuilds the ROS workspace, then runs Nav2 navigation and its sidekick nodes. |
| `cleanbackend.sh`| Kills ROS zombie processes and cleans up leftover build files (like build, install, and log folders). |
| `entrypoint.sh` | The default script that runs when the Docker container first starts up. |
