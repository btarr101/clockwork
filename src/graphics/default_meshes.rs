use super::{ MeshData, Vertex, Index };

const FRONT_BOTTOM_LEFT: Index = 0;
const FRONT_BOTTOM_RIGHT: Index = 1;
const FRONT_TOP_LEFT: Index = 2;
const FRONT_TOP_RIGHT: Index = 3;

const BACK_BOTTOM_LEFT: Index = 4;
const BACK_BOTTOM_RIGHT: Index = 5;
const BACK_TOP_LEFT: Index = 6;
const BACK_TOP_RIGHT: Index = 7;

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
    indices: &[
        FRONT_BOTTOM_LEFT,
        FRONT_BOTTOM_RIGHT,
        FRONT_TOP_RIGHT,

        FRONT_BOTTOM_LEFT,
        FRONT_TOP_RIGHT,
        FRONT_TOP_LEFT,
    ],
};

pub const CUBE_MESH_DATA: MeshData = MeshData {
    vertices: &[
        // Front Bottom Left
        Vertex {
            position: glam::vec3(-0.5, -0.5, 0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 1.0),
        },
        // Front Bottom Right
        Vertex {
            position: glam::vec3(0.5, -0.5, 0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 1.0),
        },
        // Front Top Left
        Vertex {
            position: glam::vec3(-0.5, 0.5, 0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 0.0),
        },
        // Front Top Right
        Vertex {
            position: glam::vec3(0.5, 0.5, 0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(1.0, 0.0),
        },

        // Back Bottom Left
        Vertex {
            position: glam::vec3(-0.5, -0.5, -0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 0.0),
        },
        // Back Bottom Right
        Vertex {
            position: glam::vec3(0.5, -0.5, -0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 0.0),
        },
        // Back Top Left
        Vertex {
            position: glam::vec3(-0.5, 0.5, -0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 1.0),
        },
        // Back Top Right
        Vertex {
            position: glam::vec3(0.5, 0.5, -0.5),
            normal: glam::vec3(0.0, 0.0, 1.0),
            texture_coordinates: glam::vec2(0.0, 1.0),
        },
    ],
    indices: &[
        // front face
        FRONT_BOTTOM_LEFT,
        FRONT_BOTTOM_RIGHT,
        FRONT_TOP_RIGHT,

        FRONT_BOTTOM_LEFT,
        FRONT_TOP_RIGHT,
        FRONT_TOP_LEFT,

        // back face
        BACK_TOP_RIGHT,
        BACK_BOTTOM_RIGHT,
        BACK_BOTTOM_LEFT,

        BACK_TOP_LEFT,
        BACK_TOP_RIGHT,
        BACK_BOTTOM_LEFT,

        // top face
        FRONT_TOP_LEFT,
        FRONT_TOP_RIGHT,
        BACK_TOP_RIGHT,

        FRONT_TOP_LEFT,
        BACK_TOP_RIGHT,
        BACK_TOP_LEFT,

        // left face
        BACK_BOTTOM_LEFT,
        FRONT_BOTTOM_LEFT,
        FRONT_TOP_LEFT,

        BACK_BOTTOM_LEFT,
        FRONT_TOP_LEFT,
        BACK_TOP_LEFT,
    ],
};
