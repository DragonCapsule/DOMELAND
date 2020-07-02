pub mod idle;
pub mod jump;
pub mod run;

// Reexports
pub use self::{idle::IdleAnimation, jump::JumpAnimation, run::RunAnimation};

use super::{Bone, Skeleton};
use crate::render::FigureBoneData;
use common::comp::{self};
use vek::{Mat4, Vec3};

#[derive(Clone)]
pub struct FishMediumSkeleton {
    head: Bone,
    torso: Bone,
    rear: Bone,
    tail: Bone,
    fin_l: Bone,
    fin_r: Bone,
}

impl FishMediumSkeleton {
    pub fn new() -> Self {
        Self {
            head: Bone::default(),
            torso: Bone::default(),
            rear: Bone::default(),
            tail: Bone::default(),
            fin_l: Bone::default(),
            fin_r: Bone::default(),
        }
    }
}

impl Skeleton for FishMediumSkeleton {
    type Attr = SkeletonAttr;

    fn bone_count(&self) -> usize { 6 }

    fn compute_matrices<F: FnMut(Mat4<f32>) -> FigureBoneData>(
        &self,
        mut make_bone: F,
    ) -> ([FigureBoneData; 16], Vec3<f32>) {
        let torso_mat = self.torso.compute_base_matrix();
        let rear_mat = self.rear.compute_base_matrix();

        (
            [
                make_bone(self.head.compute_base_matrix() * torso_mat),
                make_bone(torso_mat),
                make_bone(rear_mat * torso_mat),
                make_bone(self.tail.compute_base_matrix() * rear_mat),
                make_bone(self.fin_l.compute_base_matrix() * rear_mat),
                make_bone(self.fin_r.compute_base_matrix() * rear_mat),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
            ],
            Vec3::default(),
        )
    }

    fn interpolate(&mut self, target: &Self, dt: f32) {
        self.head.interpolate(&target.head, dt);
        self.torso.interpolate(&target.torso, dt);
        self.rear.interpolate(&target.rear, dt);
        self.tail.interpolate(&target.tail, dt);
        self.fin_l.interpolate(&target.fin_l, dt);
        self.fin_r.interpolate(&target.fin_r, dt);
    }
}
pub struct SkeletonAttr;

impl<'a> std::convert::TryFrom<&'a comp::Body> for SkeletonAttr {
    type Error = ();

    fn try_from(body: &'a comp::Body) -> Result<Self, Self::Error> {
        match body {
            comp::Body::FishMedium(body) => Ok(SkeletonAttr::from(body)),
            _ => Err(()),
        }
    }
}

impl Default for SkeletonAttr {
    fn default() -> Self { Self }
}

impl<'a> From<&'a comp::fish_medium::Body> for SkeletonAttr {
    fn from(_body: &'a comp::fish_medium::Body) -> Self { Self }
}