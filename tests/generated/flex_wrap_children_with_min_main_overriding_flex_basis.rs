#[test]
fn flex_wrap_children_with_min_main_overriding_flex_basis() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_basis: stretch::style::Dimension::Points(50f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                flex_basis: stretch::style::Dimension::Points(50f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(55f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 55f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 55f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 50f32);
}
