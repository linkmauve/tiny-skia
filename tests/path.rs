use tiny_skia::*;

#[test]
fn empty() {
    let pb = PathBuilder::new();
    assert!(pb.finish().is_none());
}

#[test]
fn line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 20.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);

    assert_eq!(format!("{:?}", path),
               "Path { segments: \"M 10 20 L 30 40\", \
                bounds: Bounds { left: 10, top: 20, right: 30, bottom: 40 } }");
}

#[test]
fn no_move_before_line() {
    let mut pb = PathBuilder::new();
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(0.0, 0.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
       PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
       PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);
}

#[test]
fn no_move_before_quad() {
    let mut pb = PathBuilder::new();
    pb.quad_to(40.0, 30.0, 60.0, 75.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(0.0, 0.0, 60.0, 75.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
       PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
       PathSegment::QuadTo(Point::from_xy(40.0, 30.0), Point::from_xy(60.0, 75.0)),
    ]);
}

#[test]
fn no_move_before_cubic() {
    let mut pb = PathBuilder::new();
    pb.cubic_to(40.0, 30.0, 60.0, 75.0, 33.0, 66.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(0.0, 0.0, 60.0, 75.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(0.0, 0.0)),
        PathSegment::CubicTo(Point::from_xy(40.0, 30.0), Point::from_xy(60.0, 75.0), Point::from_xy(33.0, 66.0)),
    ]);
}

#[test]
fn no_move_before_close() {
    let mut pb = PathBuilder::new();
    pb.close();
    assert!(pb.finish().is_none());
}

#[test]
fn double_close() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(20.0, 10.0);
    pb.line_to(20.0, 20.0);
    pb.close();
    pb.close();
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 10.0, 20.0, 20.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 20.0)),
        PathSegment::Close,
    ]);
}

#[test]
fn double_move_to_1() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.move_to(30.0, 40.0);
    assert!(pb.finish().is_none());
}

#[test]
fn double_move_to_2() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.move_to(20.0, 10.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(20.0, 10.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
    ]);
}

#[test]
fn two_contours() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.move_to(100.0, 200.0);
    pb.line_to(300.0, 400.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 20.0, 300.0, 400.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::MoveTo(Point::from_xy(100.0, 200.0)),
        PathSegment::LineTo(Point::from_xy(300.0, 400.0)),
    ]);
}

#[test]
fn two_closed_contours() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.close();
    pb.move_to(100.0, 200.0);
    pb.line_to(300.0, 400.0);
    pb.close();
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 20.0, 300.0, 400.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::Close,
        PathSegment::MoveTo(Point::from_xy(100.0, 200.0)),
        PathSegment::LineTo(Point::from_xy(300.0, 400.0)),
        PathSegment::Close,
    ]);
}

#[test]
fn line_after_close() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    pb.close();
    pb.line_to(20.0, 20.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 20.0, 30.0, 40.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(30.0, 40.0)),
        PathSegment::Close,
        PathSegment::MoveTo(Point::from_xy(10.0, 20.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 20.0)),
    ]);
}

#[test]
fn hor_line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(20.0, 10.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 10.0, 20.0, 10.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(20.0, 10.0)),
    ]);
}

#[test]
fn ver_line() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 10.0);
    pb.line_to(10.0, 20.0);
    let path = pb.finish().unwrap();

    assert_eq!(path.bounds(), Bounds::from_ltrb(10.0, 10.0, 10.0, 20.0).unwrap());
    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(10.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(10.0, 20.0)),
    ]);
}

#[test]
fn translate() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(&Transform::from_translate(10.0, 20.0).unwrap()).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 40.0)),
        PathSegment::LineTo(Point::from_xy(40.0, 60.0)),
    ]);
}

#[test]
fn scale() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(&Transform::from_scale(2.0, 0.5).unwrap()).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(20.0, 10.0)),
        PathSegment::LineTo(Point::from_xy(60.0, 20.0)),
    ]);
}

#[test]
fn transform() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let mut path = pb.finish().unwrap();

    path = path.transform(&Transform::from_row(2.0, 0.3, 0.7, 0.5, 10.0, 20.0).unwrap()).unwrap();

    assert_eq!(path.segments().collect::<Vec<_>>(), &[
        PathSegment::MoveTo(Point::from_xy(36.0, 55.199997)),
        PathSegment::LineTo(Point::from_xy(82.0, 97.399994)),
    ]);
}


#[test]
fn invalid_transform() {
    let mut pb = PathBuilder::new();
    pb.move_to(10.0, 20.0);
    pb.line_to(30.0, 40.0);
    let path = pb.finish().unwrap();

    // will produce infinity
    assert_eq!(path.transform(&Transform::from_scale(std::f32::MAX, std::f32::MAX).unwrap()), None);
}
