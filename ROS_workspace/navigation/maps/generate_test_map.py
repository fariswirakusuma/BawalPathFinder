import os
from ament_index_python.packages import get_package_share_directory

def create_map():
    # Dimensions: 200x200 pixels
    width, height = 200, 200
    resolution = 0.05
    
    # Initialize a completely open, white workspace (255 = Free space)
    pixels = [255] * (width * height)
    
    # Create a solid square obstacle block in the center (0 = Occupied/Obstacle)
    for y in range(80, 120):
        for x in range(80, 120):
            pixels[y * width + x] = 0

    # Locate the global installation directory for the navigation package
    try:
        package_share_dir = get_package_share_directory('navigation')
        map_dir = os.path.join(package_share_dir, 'maps')
    except Exception:
        # Fallback if running outside a sourced ROS environment during build time
        map_dir = os.path.dirname(os.path.abspath(__file__))
    
    os.makedirs(map_dir, exist_ok=True)
    
    # 1. Simpan PGM dengan nama "map.pgm"
    pgm_filename = "map.pgm"
    pgm_path = os.path.join(map_dir, pgm_filename)
    with open(pgm_path, "wb") as f:
        f.write(b"P5\n")
        f.write(f"{width} {height}\n".encode())
        f.write(b"255\n")
        f.write(bytearray(pixels))
        
    # 2. Simpan YAML dengan nama "map.yaml"
    yaml_path = os.path.join(map_dir, "map.yaml")
    origin_x = -(width * resolution) / 2.0
    origin_y = -(height * resolution) / 2.0
    
    yaml_content = f"""image: {pgm_filename}
resolution: {resolution}
origin: [{origin_x}, {origin_y}, 0.0]
occupied_thresh: 0.65
free_thresh: 0.25
negate: 0
"""
    with open(yaml_path, "w") as f:
        f.write(yaml_content)

if __name__ == "__main__":
    create_map()