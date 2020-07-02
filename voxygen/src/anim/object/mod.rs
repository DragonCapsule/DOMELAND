use super::Skeleton;
use crate::render::FigureBoneData;
use vek::*;

#[derive(Clone)]
pub struct ObjectSkeleton;
pub struct SkeletonAttr;

impl ObjectSkeleton {
    pub fn new() -> Self { Self {} }
}

const SCALE: f32 = 1.0 / 11.0;

impl Skeleton for ObjectSkeleton {
    type Attr = SkeletonAttr;

    fn bone_count(&self) -> usize { 1 }

    fn compute_matrices<F: FnMut(Mat4<f32>) -> FigureBoneData>(
        &self,
        mut make_bone: F,
    ) -> ([FigureBoneData; 16], Vec3<f32>) {
        (
            [
                make_bone(Mat4::scaling_3d(Vec3::broadcast(SCALE))),
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
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
            ],
            Vec3::default(),
        )
    }

    fn interpolate(&mut self, _target: &Self, _dt: f32) {}
}