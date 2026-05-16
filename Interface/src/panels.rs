
use crate::simulation_3d::urdf_loader;
fn render_file_selection_panel(&mut self, ui: &mut UiCtx) {
    let selected_file = "simple_drone.urdf"; 

    ui.label(format!("Selected: {}", selected_file));

    if ui.button("Generate URDF to ROS Workspace").clicked() {
        match urdf_loader::generate_urdf_to_workspace(selected_file) {
            Ok(_) => {
                println!("Generation successful!");
            }
            Err(e) => {
                eprintln!("Failed to generate URDF: {}", e);
            }
        }
    }
}