use egui::{Color32, FontId, Id, Pos2, Rect, Sense, Stroke, Ui, Vec2};

const ROUNDING: f32 = 5.0;

/// Draws a task and its children.
///
/// Parameters:
/// - `ui`: the egui UI handle.
/// - `painter`: the egui painter.
/// - `task`: the task to draw (mutable in case you need to update child state).
/// - `pos`: the center position for the parent task.
/// - `radii`: half the dimensions of the task rectangle.
/// - `ui_center`: the center of the available UI area.
/// - `is_selected`: whether the parent task is selected.
pub fn draw_task_with_children(
    ui: &mut Ui,
    painter: &egui::Painter,
    task: &mut crate::Task,
    pos: Pos2,
    radii: Vec2,
    ui_center: Pos2,
    is_selected: bool,
) {
    // Draw the parent task.
    let rect = Rect::from_center_size(pos, radii * 2.0);
    let response = ui.interact(rect, Id::new(task.name.clone()), Sense::click());
    if response.clicked() {
        // Toggle selection or update state externally.
        // (For example, you might update a selected_task_id in your app code.)
    }
    painter.rect(
        rect,
        ROUNDING,
        Color32::WHITE,
        Stroke::new(2.0, Color32::BLACK),
    );
    if is_selected {
        painter.rect(
            rect,
            ROUNDING,
            Color32::TRANSPARENT,
            Stroke::new(5.0, Color32::BLUE),
        );
    }
    painter.text(
        rect.center(),
        egui::Align2::CENTER_BOTTOM,
        format!("{}", task.name),
        FontId::proportional(16.0),
        Color32::BLACK,
    );
    painter.text(
        rect.center(),
        egui::Align2::CENTER_TOP,
        format!("{}", task.estimate),
        FontId::proportional(16.0),
        Color32::BLACK,
    );

    // --- Draw children in a half circle away from the UI center ---
    if !task.children.is_empty() {
        let num_children = task.children.len();
        // Compute the base angle so that the half-circle faces away from the UI center.
        let base_angle = (pos - ui_center).angle();
        let arc_span = std::f32::consts::PI; // 180°
        let distance_from_parent = 200.0; // Adjust as needed.
        for (j, child) in task.children.iter_mut().enumerate() {
            // Compute child angle along the half circle.
            let fraction = if num_children > 1 {
                j as f32 / (num_children - 1) as f32
            } else {
                0.5
            };
            let child_angle = base_angle - (arc_span / 2.0) + fraction * arc_span;
            // Position the child relative to the parent's center.
            let child_pos = Pos2::new(
                pos.x + distance_from_parent * child_angle.cos(),
                pos.y + distance_from_parent * child_angle.sin(),
            );
            let child_rect = Rect::from_center_size(child_pos, radii * 2.0);

            // Handle child click.
            let child_response =
                ui.interact(child_rect, Id::new(child.name.clone()), Sense::click());
            if child_response.clicked() {
                // Update state externally if needed.
            }

            // Draw child rectangle and text.
            painter.rect(
                child_rect,
                ROUNDING,
                Color32::WHITE,
                Stroke::new(2.0, Color32::BLACK),
            );
            // (If you track selection on children, you can add a blue outline similarly.)
            painter.text(
                child_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{}", child.name),
                FontId::proportional(16.0),
                Color32::BLACK,
            );

            // --- Draw line from closest edges between parent's rect and child's rect ---
            let parent_center = rect.center();
            let child_center = child_rect.center();
            let dir = (child_center - parent_center).normalized();

            // Calculate parent's edge intersection.
            let parent_half = Vec2::new(rect.width() / 2.0, rect.height() / 2.0);

            let par_x = parent_half.x
                / if dir.x.abs() < f32::EPSILON {
                    f32::INFINITY
                } else {
                    dir.x.abs()
                };

            let scale_parent = par_x.min(
                parent_half.y
                    / if dir.y.abs() < f32::EPSILON {
                        f32::INFINITY
                    } else {
                        dir.y.abs()
                    },
            );
            let parent_edge = parent_center + dir * scale_parent;

            // Calculate child's edge intersection.
            let rev_dir = -dir;
            let child_half = Vec2::new(child_rect.width() / 2.0, child_rect.height() / 2.0);

            let child_x = child_half.x
                / if rev_dir.x.abs() < f32::EPSILON {
                    f32::INFINITY
                } else {
                    rev_dir.x.abs()
                };
            let scale_child = child_x.min(
                child_half.y
                    / if rev_dir.y.abs() < f32::EPSILON {
                        f32::INFINITY
                    } else {
                        rev_dir.y.abs()
                    },
            );

            let child_edge = child_center + rev_dir * scale_child;

            if parent_edge.x == child_edge.x {
                let dir = child_edge.y - parent_edge.y;
                let parent_line_edge = parent_edge
                    + if dir > 0.0 {
                        Vec2::new(0.0, rect.height() / 2.0)
                    } else {
                        Vec2::new(0.0, -rect.height() / 2.0)
                    };
                println!("parent_edge after: {:?}", parent_edge);
                let child_line_edge = child_edge
                    + if dir > 0.0 {
                        Vec2::new(0.0, -child_rect.height() / 2.0)
                    } else {
                        Vec2::new(0.0, child_rect.height() / 2.0)
                    };
                /*
                painter.circle(
                    parent_line_edge,
                    5.0,
                    Color32::BLUE,
                    Stroke::new(1.0, Color32::DARK_GRAY),
                );
                painter.circle(
                    child_line_edge,
                    5.0,
                    Color32::RED,
                    Stroke::new(1.0, Color32::DARK_GRAY),
                ); */
                // Draw a straight line.
                painter.line_segment(
                    [parent_line_edge, child_line_edge],
                    Stroke::new(1.5, Color32::DARK_GRAY),
                );
            } else if parent_edge.y == child_edge.y {
                let dir = child_edge.x - parent_edge.x;
                let parent_line_edge = parent_edge
                    + if dir > 0.0 {
                        Vec2::new(rect.width() / 2.0, 0.0)
                    } else {
                        Vec2::new(-rect.width() / 2.0, 0.0)
                    };
                let child_line_edge = child_edge
                    + if dir > 0.0 {
                        Vec2::new(-child_rect.width() / 2.0, 0.0)
                    } else {
                        Vec2::new(child_rect.width() / 2.0, 0.0)
                    };
                /*
                painter.circle(
                    parent_line_edge,
                    5.0,
                    Color32::BLUE,
                    Stroke::new(1.0, Color32::DARK_GRAY),
                );
                painter.circle(
                    child_line_edge,
                    5.0,
                    Color32::RED,
                    Stroke::new(1.0, Color32::DARK_GRAY),
                ); */
                // Draw a straight line.
                painter.line_segment(
                    [parent_line_edge, child_line_edge],
                    Stroke::new(1.5, Color32::DARK_GRAY),
                );
            } else {
                // Draw the connecting line.
                painter.line_segment(
                    [parent_edge, child_edge],
                    Stroke::new(1.5, Color32::DARK_GRAY),
                );
            }
        }
    }
}
