#[test]
fn rounding_total_fractial() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 0.7f32,
                flex_basis: stretch::style::Dimension::Points(50.3f32),
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(20.3f32),
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
                flex_grow: 1.6f32,
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1.1f32,
                size: stretch::geometry::Size {
                    height: stretch::style::Dimension::Points(10.7f32),
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
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(87.4f32),
                    height: stretch::style::Dimension::Points(113.4f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 87f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 113f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 87f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 59f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 87f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 30f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 59f32);
    assert_eq!(stretch.layout(node2).unwrap().size.width, 87f32);
    assert_eq!(stretch.layout(node2).unwrap().size.height, 24f32);
    assert_eq!(stretch.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node2).unwrap().location.y, 89f32);
}
