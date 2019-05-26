// A dictionnary of available shaders
// For now vulkano only has a hacky way to create shaders so we also have a hacky way to initialize shaders

//use std::collections::HashMap;

/*
pub struct Shaders {
    shaders: HashMap<&'static str, ()>,
}
*/

// For now we have to load all the shaders by hand

use std::sync::Arc;

use vulkano::device::Device;

pub mod basic_triangle_vert {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/basic_triangle.vert"
    }
}

pub mod basic_triangle_frag {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "shaders/basic_triangle.frag"
    }
}

/*
impl Shaders {
    // Loads all the glsl files located in the *shaders* root folder (not in the *src* subfolder)
    // We somehow need the type information so we cannot do this now
    pub fn load_shaders(device: Arc<Device>) -> Self {
        let mut shaders = Shaders { shaders: HashMap::new() };

        // locate the *shaders* directory
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let mut full_path = PathBuf::from(&root);
        full_path.push("shaders");

        // Load all the files in the directory
        for shader in read_dir(full_path).expect("Cannot read into shaders/ directory") {
            let shader_name = shader.expect("Can't read into shaders/ directory").file_name();
            let mut shader_file = PathBuf::from("shaders/");
            shader_file.push(shader_name);

            shaders.shaders.insert(shader_base_name, load_shader_obj!(shader_name));
        }

        shaders
    }
}
*/
