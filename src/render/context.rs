use nalgebra::*;

#[derive(Clone,Copy)]
pub struct RenderContext {

    aspect_ratio: f32,
    region_matrix: Matrix3<f32>
}

impl RenderContext {

    pub fn full(aspect_ratio: f32) -> Self {
        Self {
            aspect_ratio,
            region_matrix: Matrix3::identity()
        }
    }

    pub fn sub(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        let sub_aspect_ratio = self.aspect_ratio * (max_y - min_y) / (max_x - min_x);

        let right_conversion_matrix = Matrix3::new_nonuniform_scaling(&Vector2::new(max_x - min_x, max_y - min_y));
        let left_conversion_matrix = Matrix3::new_translation(&Vector2::new(min_x, min_y));
        let conversion_matrix = left_conversion_matrix * right_conversion_matrix;
        let sub_region_matrix = conversion_matrix * self.region_matrix;

        Self {
            aspect_ratio: sub_aspect_ratio,
            region_matrix: sub_region_matrix
        }
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_region_matrix(&self) -> Matrix3<f32> {
        self.region_matrix
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sub() {

        let root = RenderContext::full(1.5);

        let middle_half = root.sub(0.25, 0.25, 0.75, 0.75);

        assert_eq!(1.5, middle_half.aspect_ratio);
        {
            let test_point = Vector3::new(0.2, 0.5, 1.0);
            let dest_point = middle_half.region_matrix * test_point;
            assert_eq!(dest_point[0], 0.35);
            assert_eq!(dest_point[1], 0.5);
        }

        let middle_left = middle_half.sub(0.0, 0.0, 0.5, 1.0);
        assert_eq!(3.0, middle_left.aspect_ratio);
        {
            let test_point = Vector3::new(0.5, 0.5, 1.0);
            let dest_point = middle_left.region_matrix * test_point;
            assert_eq!(0.375, dest_point[0]);
            assert_eq!(0.5, dest_point[1]);
        }
    }
}