import os
import random
import math
from PIL import Image
from faker import Faker

def create_random_map():
    fake = Faker()
    width, height = 200, 200
    resolution = 0.05
    
    map_dir = os.path.dirname(os.path.abspath(__file__))
    
    img = Image.new('L', (width, height), 255)
    num_obstacles = random.randint(15, 25)
    start_px, start_py = 20, 20
    goal_px, goal_py = 180, 180
    safe_radius = 30
    
    for _ in range(num_obstacles):
        obs_w = random.randint(15, 40)
        obs_h = random.randint(15, 40)
        
        start_x = random.randint(0, width - obs_w)
        start_y = random.randint(0, height - obs_h)
        
        center_x = start_x + obs_w / 2
        center_y = start_y + obs_h / 2
        
        dist_start = math.hypot(center_x - start_px, center_y - start_py)
        dist_goal = math.hypot(center_x - goal_px, center_y - goal_py)
        
        if dist_start < safe_radius or dist_goal < safe_radius:
            continue
            
        for y in range(start_y, start_y + obs_h):
            for x in range(start_x, start_x + obs_w):
                img.putpixel((x, y), 0)
    
    base_name = f"map_{fake.word()}_{random.randint(100,999)}"
    png_filename = f"{base_name}.png"
    yaml_filename = f"{base_name}.yaml"
    
    png_path = os.path.join(map_dir, png_filename)
    img.save(png_path)
        
    yaml_path = os.path.join(map_dir, yaml_filename)
    origin_x = -(width * resolution) / 2.0
    origin_y = -(height * resolution) / 2.0
    
    yaml_content = f"image: {png_filename}\nresolution: {resolution}\norigin: [{origin_x:.2f}, {origin_y:.2f}, 0.0]\noccupied_thresh: 0.65\nfree_thresh: 0.25\nnegate: 0\n"
    with open(yaml_path, "w") as f:
        f.write(yaml_content)

if __name__ == "__main__":
    create_random_map()