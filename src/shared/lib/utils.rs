use crate::shared::lib::types::{Point, Rect};

pub fn point_in_rect(point: &Point, rect: &Rect) -> bool {
    point.x >= rect.position.x 
        && point.x <= rect.position.x + rect.size.width
        && point.y >= rect.position.y 
        && point.y <= rect.position.y + rect.size.height
}

pub fn rects_intersect(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.position.x < rect2.position.x + rect2.size.width
        && rect1.position.x + rect1.size.width > rect2.position.x
        && rect1.position.y < rect2.position.y + rect2.size.height
        && rect1.position.y + rect1.size.height > rect2.position.y
}

pub fn generate_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
} 