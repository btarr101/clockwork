use std::{ f32::consts::PI, cell::RefCell };

use glam::{ Affine3A, Mat4 };

#[derive(Clone, Copy)]
pub enum Projection {
    Perspective {
        aspect: f32,
        fov: f32,
        znear: f32,
        zfar: f32,
    },
}

/// Helper for generating a view projection matrix (the model comes later)
pub struct Camera {
    pub affine: Affine3A,

    projection: Projection,
    projection_mat: RefCell<Option<Mat4>>,
}

impl Projection {
    fn to_matrix(self) -> Mat4 {
        match self {
            Projection::Perspective { aspect, fov, znear, zfar } =>
                Mat4::perspective_rh(fov, aspect, znear, zfar),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let projection = Projection::Perspective {
            aspect: 1.0,
            fov: PI / 2.0,
            znear: 0.01,
            zfar: 100.0,
        };

        Self {
            affine: Affine3A::IDENTITY,
            projection,
            projection_mat: RefCell::new(None),
        }
    }
}

impl Camera {
    /// Mutates the [Projection] of this [Camera].
    ///
    /// Since generating the projection matrix takes work, it is only regenerated if
    /// the need arises. Calling this function signals that the projection will change,
    /// therefore destroying the cached projection matrix.
    pub fn mut_projection(&mut self) -> &mut Projection {
        *self.projection_mat.borrow_mut() = None;
        &mut self.projection
    }

    /// Gets the current view projection matrix for this [Camera].
    pub fn get_view_projection_matrix(&self) -> Mat4 {
        let projection_mat = *self.projection_mat
            .borrow_mut()
            .get_or_insert_with(|| self.projection.to_matrix());
        projection_mat * self.affine.inverse()
    }
}
