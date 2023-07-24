use std::cell::RefCell;

/// Fields regarding the projection of a [Camera].
#[derive(Clone, Copy)]
pub enum Projection {
    Perspective {
        /// Aspect ratio of the target display.
        aspect: f32,
        /// Field of view.
        fov: f32,
        /// How close objects can get before they are clipped.
        znear: f32,
        /// How far objects can be before they are clipped.
        zfar: f32,
    },
}

/// Helper for generating a view projection matrix (the model comes later)
pub struct Camera {
    /// Transformation of the [Camera].
    pub affine: glam::Affine3A,
    projection: Projection,
    projection_mat: RefCell<Option<glam::Mat4>>,
}

impl Projection {
    fn to_matrix(self) -> glam::Mat4 {
        match self {
            Projection::Perspective { aspect, fov, znear, zfar } =>
                glam::Mat4::perspective_rh(fov, aspect, znear, zfar),
        }
    }
}

impl Camera {
    /// Creates a new [Camera] with the given projection.
    pub fn new(affine: glam::Affine3A, projection: Projection) -> Self {
        Self {
            affine,
            projection,
            projection_mat: RefCell::new(None),
        }
    }

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
    pub fn get_view_projection_matrix(&self) -> glam::Mat4 {
        let projection_mat = *self.projection_mat
            .borrow_mut()
            .get_or_insert_with(|| self.projection.to_matrix());
        projection_mat * self.affine.inverse()
    }
}
