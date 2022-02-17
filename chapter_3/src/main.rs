#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;
    width * height
}

fn square(point: Point, diameter: f32) -> Rectangle {
    let bottom_right_x = point.x + diameter;
    let bottom_right_y = point.y + diameter;
    Rectangle {
        top_left: point,
        bottom_right: Point {
            x: bottom_right_x,
            y: bottom_right_y,
        },
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 3.0 },
        bottom_right: Point { x: 3.0, y: 0.0 },
    };
    println!("{}", rect_area(rect));
    println!("{:?}", square(Point { x: 3.0, y: 3.3 }, 5.0));
}
