use super::{ MeshData, Vertex };

/// Mesh data for a square with width and height of 1.0.
pub const QUAD_MESH_DATA: MeshData = MeshData {
    vertices: &[
        // Bottom Left
        Vertex {
            position: glam::vec3(-0.5, -0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 1.0),
        },
        // Bottom Right
        Vertex {
            position: glam::vec3(0.5, -0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 1.0),
        },
        // Top Left
        Vertex {
            position: glam::vec3(-0.5, 0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 0.0),
        },
        // Top Right
        Vertex {
            position: glam::vec3(0.5, 0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 0.0),
        },
    ],
    indices: &[0, 1, 3, 0, 3, 2],
};

pub const CUBE_MESH_DATA: MeshData = MeshData {
    vertices: &[
        // Bottom Left
        Vertex {
            position: glam::vec3(-0.5, -0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 1.0),
        },
        // Bottom Right
        Vertex {
            position: glam::vec3(0.5, -0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 1.0),
        },
        // Top Left
        Vertex {
            position: glam::vec3(-0.5, 0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 0.0),
        },
        // Top Right
        Vertex {
            position: glam::vec3(0.5, 0.5, 0.0),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 0.0),
        },
    ],
    indices: &[0, 1, 3, 0, 3, 2],
};
