use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

pub const ROUNDING: f32 = 5.0;
pub const RADII: Vec2 = Vec2::new(75.0, 25.0);

#[derive(Clone)]
pub struct TaskPosition {
    task_id: String,
    screen_center: Pos2,
    parent_rect: Rect,
    is_child: bool,
    task_index: usize,
    tasks_length: usize,
    depth_level: usize,
}

impl TaskPosition {
    pub fn new(
        task_id: String,
        screen_center: Pos2,
        parent_rect: Rect,
        is_child: bool,
        task_index: usize,
        tasks_length: usize,
        depth_level: usize,
    ) -> Self {
        Self {
            task_id,
            screen_center,
            parent_rect,
            is_child,
            task_index,
            tasks_length,
            depth_level,
        }
    }
    pub fn new_child(&self, parent: Rect, task_index: usize, tasks_length: usize) -> TaskPosition {
        TaskPosition {
            task_id: self.task_id.clone(),
            screen_center: self.screen_center,
            parent_rect: parent,
            is_child: true,
            task_index,
            tasks_length,
            depth_level: self.depth_level + 1,
        }
    }
}

pub fn draw_task(
    painter: &egui::Painter,
    task: &crate::Task,
    radius: f32,
    placed_positions: &[Pos2],
    selected_task_id: Option<&str>,
    task_position: TaskPosition,
) -> (Pos2, i32) {
    let radii = RADII;
    //let radius = parent_rect.width().min(parent_rect.height()) * 0.3;
    let (screen_center, parent_rect, is_child, index, max_size, depth_level) = (
        task_position.screen_center,
        task_position.parent_rect,
        task_position.is_child,
        task_position.task_index,
        task_position.tasks_length,
        task_position.depth_level,
    );

    let rect = if is_child {
        get_child_rect(
            index,
            max_size,
            parent_rect.center(),
            screen_center,
            radii,
            placed_positions,
        )
    } else {
        get_rectangle_calculated(
            index,
            max_size,
            &parent_rect,
            radius,
            radii,
            placed_positions,
        )
    };

    paint_rectangle(
        painter,
        rect,
        selected_task_id == Some(task.id.as_str()),
        task.name.clone(),
        Some(task.estimate.to_string()),
        depth_level,
    );

    draw_line(painter, parent_rect, rect);

    let mut child_sums = 0;

    if !task.children.is_empty() {
        let mut child_positions = Vec::new();
        for (j, child_task) in task.children.iter().enumerate() {
            let task_position_child = task_position.new_child(rect, j, task.children.len());
            let (child_pos, child_sum) = draw_task(
                painter,
                child_task,
                radius / 1.5,
                &child_positions,
                selected_task_id,
                task_position_child,
            );
            child_positions.push(child_pos);
            child_sums += child_sum;
        }
        draw_sum(painter, child_sums, rect);
    }

    child_sums += task.estimate;

    (rect.center(), child_sums)
}

pub fn draw_line(painter: &egui::Painter, from_rect: Rect, to_rect: Rect) {
    let parent_center = from_rect.center();
    let child_center = to_rect.center();
    let dir = (child_center - parent_center).normalized();

    // Calculate parent's edge intersection.
    let parent_half = Vec2::new(from_rect.width() / 2.0, from_rect.height() / 2.0);

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
    let child_half = Vec2::new(to_rect.width() / 2.0, to_rect.height() / 2.0);

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
                Vec2::new(0.0, from_rect.height() / 2.0)
            } else {
                Vec2::new(0.0, -from_rect.height() / 2.0)
            };

        let child_line_edge = child_edge
            + if dir > 0.0 {
                Vec2::new(0.0, -to_rect.height() / 2.0)
            } else {
                Vec2::new(0.0, to_rect.height() / 2.0)
            };

        // Draw a straight line.
        painter.line_segment(
            [parent_line_edge, child_line_edge],
            Stroke::new(1.5, Color32::DARK_GRAY),
        );
    } else if parent_edge.y == child_edge.y {
        let dir = child_edge.x - parent_edge.x;
        let parent_line_edge = parent_edge
            + if dir > 0.0 {
                Vec2::new(from_rect.width() / 2.0, 0.0)
            } else {
                Vec2::new(-from_rect.width() / 2.0, 0.0)
            };
        let child_line_edge = child_edge
            + if dir > 0.0 {
                Vec2::new(-to_rect.width() / 2.0, 0.0)
            } else {
                Vec2::new(to_rect.width() / 2.0, 0.0)
            };
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

fn draw_sum(painter: &egui::Painter, sum: i32, parent_rect: Rect) {
    let mut position = parent_rect.center();
    position.x -= parent_rect.width() / 2.0;
    position.y -= parent_rect.height() / 2.0;

    //painter.circle_stroke(position, 20.0, Stroke::new(1.5, Color32::KHAKI));
    painter.circle(
        position,
        20.0,
        Color32::LIGHT_BLUE,
        Stroke::new(1.5, Color32::KHAKI),
    );
    painter.text(
        position,
        Align2::CENTER_CENTER,
        sum.to_string(),
        FontId::proportional(16.0),
        Color32::BLACK,
    );
}

pub fn paint_rectangle(
    painter: &egui::Painter,
    rect: Rect,
    selected: bool,
    first_row: String,
    second_row: Option<String>,
    depth_level: usize,
) {
    painter.rect(
        rect,
        ROUNDING,
        if depth_level > 0 {
            Color32::LIGHT_GRAY
        } else {
            Color32::WHITE
        },
        Stroke::new(2.0, Color32::BLACK),
    );
    if selected {
        painter.rect(
            rect,
            ROUNDING,
            Color32::TRANSPARENT,
            Stroke::new(5.0, Color32::BLUE),
        );
    }
    painter.text(
        rect.center(),
        if second_row.is_some() {
            egui::Align2::CENTER_BOTTOM
        } else {
            egui::Align2::CENTER_CENTER
        },
        first_row.to_string(),
        FontId::proportional(16.0),
        Color32::BLACK,
    );
    if let Some(second_row) = second_row {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_TOP,
            second_row,
            FontId::proportional(16.0),
            Color32::BLACK,
        );
    }
}

fn adjust_position(
    angle: f32,
    radius: f32,
    position: Pos2,
    parent: &Rect,
    placed_positions: &[Pos2],
) -> Pos2 {
    // Adjust position to avoid overlap.
    let mut current_radius = radius;
    let mut adjusted_pos = position;
    let safe_distance = (75.0_f32.powi(2) + 25.0_f32.powi(2)).sqrt() * 2.0;
    while placed_positions
        .iter()
        .any(|&p| p.distance(adjusted_pos) < safe_distance)
    {
        current_radius += 10.0;
        adjusted_pos = Pos2::new(
            parent.center().x + current_radius * angle.cos(),
            parent.center().y + current_radius * angle.sin(),
        );
    }
    adjusted_pos
}

fn get_rectangle_calculated(
    index: usize,
    count: usize,
    parent_rect: &Rect,
    radius: f32,
    radii: Vec2,
    placed_positions: &[Pos2],
) -> Rect {
    let angle = index as f32 / count as f32 * std::f32::consts::TAU;
    let position = Pos2::new(
        parent_rect.center().x + radius * angle.cos(),
        parent_rect.center().y + radius * angle.sin(),
    );

    let adjusted_pos = adjust_position(angle, radius, position, parent_rect, placed_positions);

    // Draw the parent task.
    Rect::from_center_size(adjusted_pos, radii * 2.0)
}

fn get_child_rect(
    child_index: usize,
    child_count: usize,
    parent: Pos2,
    center: Pos2,
    radii: Vec2,
    placed_positions: &[Pos2],
) -> Rect {
    // Compute the base angle so that the half-circle faces away from the UI center.
    let base_angle = (parent - center).angle();
    let arc_span = std::f32::consts::PI; // 180°
    let distance_from_parent = 200.0; // Adjust as needed.

    // Compute child angle along the half circle.
    let fraction = if child_count > 1 {
        child_index as f32 / (child_count - 1) as f32
    } else {
        0.5
    };
    let child_angle = base_angle - (arc_span / 2.0) + fraction * arc_span;
    // Position the child relative to the parent's center.
    let child_pos = Pos2::new(
        parent.x + distance_from_parent * child_angle.cos(),
        parent.y + distance_from_parent * child_angle.sin(),
    );

    let adjusted_pos = adjust_position(
        child_angle,
        distance_from_parent,
        child_pos,
        &Rect::from_center_size(parent, radii * 2.0),
        placed_positions,
    );

    Rect::from_center_size(adjusted_pos, radii * 2.0)
}
