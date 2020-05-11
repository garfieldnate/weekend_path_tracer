use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    utils::random_int_in_range,
};
use std::cmp::Ordering::Equal;
use std::{cmp::Ordering, sync::Arc};

#[derive(Clone, Debug)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounds: AABB,
}
impl BvhNode {
    pub fn new_from_hittable(list: &HittableList, time_0: f64, time_1: f64) -> Self {
        BvhNode::new(&list.objects, 0, list.objects.len(), time_0, time_1)
    }
    fn new(
        objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time_0: f64,
        time_1: f64,
    ) -> Self {
        let mut objects = objects.clone();
        let axis = random_int_in_range(0, 3);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if object_span == 1 {
            let single_span = objects[start].clone();
            (single_span.clone(), single_span.clone())
        } else if object_span == 2 {
            match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Greater => (objects[start].clone(), objects[start + 1].clone()),
                _ => (objects[start + 1].clone(), objects[start].clone()),
            }
        } else {
            // TODO: does this work as expected?
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            (
                Arc::new(BvhNode::new(&objects, start, mid, time_0, time_1)),
                Arc::new(BvhNode::new(&objects, mid, end, time_0, time_1)),
            )
        };

        let box_left = left.bounding_box(time_0, time_1);
        let box_right = right.bounding_box(time_0, time_1);

        if box_left.is_none() || box_right.is_none() {
            eprintln!("No bounding box in BvnNode constructor")
        }

        let bounds = box_left
            .unwrap_or(AABB::default())
            .combine(box_right.unwrap_or(AABB::default()));

        Self {
            left,
            right,
            bounds,
        }
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0., 0.);
    let box_b = b.bounding_box(0., 0.);

    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in bvh_node constructor.");
    }
    let box_a = box_a.unwrap_or(AABB::default());
    let box_b = box_b.unwrap_or(AABB::default());

    box_a.min().data[axis]
        .partial_cmp(&box_b.min().data[axis])
        .unwrap_or(Equal)
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounds.hit(&r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = match &hit_left {
            None => self.right.hit(r, t_min, t_max),
            Some(rec) => self.right.hit(r, t_min, rec.t),
        };
        // if hit_right exists, it was the closer hit
        match hit_right {
            Some(_) => hit_right,
            // hit_left may or may not exist
            None => hit_left,
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        return Some(self.bounds);
    }
}
