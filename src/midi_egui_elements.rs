use eframe::{egui, epaint::Stroke};

fn midi_status_indicator_ui(ui: &mut egui::Ui, status: &bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (_rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let mut how_on = ui.ctx().animate_bool(response.id, *status);
        if *status {
            how_on = 1.0;
        }
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * how_on, rect.height()));
        let (_dark, bright) = (0.7, 1.0);
        if how_on > 0.0 {
            let color_factor = bright * how_on;
            ui.painter().rect(
                inner_rect,
                rounding,
                egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor),
                Stroke::NONE,
            );
        }
        let status_str = if *status { "on" } else { "off" };
        let text: egui::WidgetText = status_str.into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos = rect.left_center()
            - egui::Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0)
            + egui::vec2(rect.width() / 2.0, 0.0);
        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley.paint_with_fallback_color(&ui.painter().with_clip_rect(rect), text_pos, text_color);
    }
    response
}

fn midi_value_indicator_ui(ui: &mut egui::Ui, value: u32) -> egui::Response {
    let value: u32 = value.min(127);
    let mut fill_level: f32 = value as f32 / 127.0;
    fill_level = fill_level.clamp(0.0, 1.0);
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level).max(rect.height()),
                rect.height(),
            ),
        );
        let (_dark, bright) = (0.7, 1.0);
        let color_factor = bright;
        ui.painter().rect(
            inner_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
            Stroke::NONE,
        );
        let text: egui::WidgetText = format!("{}", (fill_level * 127.0) as usize).into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos = rect.left_center() - egui::Vec2::new(0.0, galley.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);
        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley.paint_with_fallback_color(&ui.painter().with_clip_rect(rect), text_pos, text_color);
    }
    response
}

fn midi_note_status_indicator_ui(ui: &mut egui::Ui, id: u32, status: &bool) -> egui::Response {
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let mut how_on = ui.ctx().animate_bool(response.id, *status);
        if *status {
            how_on = 1.0;
        }
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * how_on, rect.height()));
        let (_dark, bright) = (0.7, 1.0);
        if how_on > 0.0 {
            let color_factor = bright * how_on;
            ui.painter().rect(
                inner_rect,
                rounding,
                egui::Color32::from(
                    egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32,
                ),
                Stroke::NONE,
            );
        }
        let text_id: egui::WidgetText = format!("{}", id).into();
        let galley_note =
            text_id.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos_note = rect.left_center() - egui::Vec2::new(0.0, galley_note.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);

        let status_str = if *status { "on" } else { "off" };
        let text: egui::WidgetText = status_str.into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos = rect.left_center()
            - egui::Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0)
            + egui::vec2(rect.width() / 2.0, 0.0);
        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley.paint_with_fallback_color(&ui.painter().with_clip_rect(rect), text_pos, text_color);
        galley_note.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_note,
            text_color,
        );
    }
    response
}

fn midi_note_status_intensity_indicator_ui(
    ui: &mut egui::Ui,
    id: u32,
    status: &bool,
    intensity: u32,
) -> egui::Response {
    let intensity: u32 = intensity.min(127);
    let mut fill_level_intensity: f32 = intensity as f32 / 127.0;
    fill_level_intensity = fill_level_intensity.clamp(0.0, 1.0);
    // TODO paint intensity rect
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let mut how_on = ui.ctx().animate_bool(response.id, *status);
        if *status {
            how_on = 1.0;
        }
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * how_on, rect.height()));
        let (_dark, bright) = (0.7, 1.0);
        if how_on > 0.0 {
            let color_factor = bright * how_on;
            ui.painter().rect(
                inner_rect,
                rounding,
                egui::Color32::from(
                    egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32,
                ),
                Stroke::NONE,
            );
        }
        let intensity_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level_intensity).max(rect.height()),
                rect.height() / 2.0,
            ),
        );
        ui.painter().rect(
            intensity_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.error_fg_color)),
            Stroke::NONE,
        );
        let text_id: egui::WidgetText = format!("{}", id).into();
        let galley_note =
            text_id.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos_note = rect.left_center() - egui::Vec2::new(0.0, galley_note.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);

        let status_str = if *status { "on" } else { "off" };
        let text: egui::WidgetText = status_str.into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos = rect.left_center()
            - egui::Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0)
            + egui::vec2(rect.width() / 2.0, 0.0);
        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley.paint_with_fallback_color(&ui.painter().with_clip_rect(rect), text_pos, text_color);
        galley_note.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_note,
            text_color,
        );
    }
    response
}

fn midi_two_value_indicator_ui(ui: &mut egui::Ui, note: u32, intensity: u32) -> egui::Response {
    let note: u32 = note.min(127);
    let intensity: u32 = intensity.min(127);
    let mut fill_level_note: f32 = note as f32 / 127.0;
    fill_level_note = fill_level_note.clamp(0.0, 1.0);
    let mut fill_level_intensity: f32 = intensity as f32 / 127.0;
    fill_level_intensity = fill_level_intensity.clamp(0.0, 1.0);
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level_note).max(rect.height()),
                rect.height(),
            ),
        );
        let (_dark, bright) = (0.7, 1.0);
        let color_factor = bright;
        ui.painter().rect(
            inner_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
            Stroke::NONE,
        );
        let text_note: egui::WidgetText = format!("{}", (fill_level_note * 127.0) as usize).into();
        let galley_note =
            text_note.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_intensity: egui::WidgetText =
            format!("{}", (fill_level_intensity * 127.0) as usize).into();
        let galley_intensity =
            text_intensity.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos_note = rect.left_center() - egui::Vec2::new(0.0, galley_note.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);
        let text_pos_intensity = rect.left_center()
            - egui::Vec2::new(
                galley_intensity.size().x / 2.0,
                galley_intensity.size().y / 2.0,
            )
            + egui::vec2(rect.width() / 2.0, 0.0);

        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley_intensity.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_intensity,
            text_color,
        );
        galley_note.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_note,
            text_color,
        );
    }
    response
}

fn midi_id_value_indicator_ui(ui: &mut egui::Ui, id: u32, intensity: u32) -> egui::Response {
    let intensity: u32 = intensity.min(127);
    let mut fill_level_intensity: f32 = intensity as f32 / 127.0;
    fill_level_intensity = fill_level_intensity.clamp(0.0, 1.0);
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level_intensity).max(rect.height()),
                rect.height(),
            ),
        );
        let (_dark, bright) = (0.7, 1.0);
        let color_factor = bright;
        ui.painter().rect(
            inner_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
            Stroke::NONE,
        );
        let text_id: egui::WidgetText = format!("{}", id).into();
        let galley_note =
            text_id.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_intensity: egui::WidgetText =
            format!("{}", (fill_level_intensity * 127.0) as usize).into();
        let galley_intensity =
            text_intensity.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos_note = rect.left_center() - egui::Vec2::new(0.0, galley_note.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);
        let text_pos_intensity = rect.left_center()
            - egui::Vec2::new(
                galley_intensity.size().x / 2.0,
                galley_intensity.size().y / 2.0,
            )
            + egui::vec2(rect.width() / 2.0, 0.0);

        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley_intensity.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_intensity,
            text_color,
        );
        galley_note.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_note,
            text_color,
        );
    }
    response
}

fn midi_id_double_precision_value_indicator_ui(
    ui: &mut egui::Ui,
    id: u32,
    intensity: u32,
) -> egui::Response {
    let intensity = intensity.min(16384) as f32 / 16384.0;
    let fill_level_intensity = intensity.clamp(0.0, 1.0);
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level_intensity).max(rect.height()),
                rect.height(),
            ),
        );
        let (_dark, bright) = (0.7, 1.0);
        let color_factor = bright;
        ui.painter().rect(
            inner_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
            Stroke::NONE,
        );
        let text_id: egui::WidgetText = format!("{}", id).into();
        let galley_note =
            text_id.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_intensity: egui::WidgetText =
            format!("{}", (fill_level_intensity * 16384.0) as usize).into();
        let galley_intensity =
            text_intensity.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos_note = rect.left_center() - egui::Vec2::new(0.0, galley_note.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);
        let text_pos_intensity = rect.left_center()
            - egui::Vec2::new(
                galley_intensity.size().x / 2.0,
                galley_intensity.size().y / 2.0,
            )
            + egui::vec2(rect.width() / 2.0, 0.0);

        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley_intensity.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_intensity,
            text_color,
        );
        galley_note.paint_with_fallback_color(
            &ui.painter().with_clip_rect(rect),
            text_pos_note,
            text_color,
        );
    }
    response
}

pub fn midi_status_indicator(status: &bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| midi_status_indicator_ui(ui, status)
}

pub fn midi_value_indicator(value: u32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| midi_value_indicator_ui(ui, value)
}

pub fn midi_note_status_indicator(note: u32, status: &bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| midi_note_status_indicator_ui(ui, note, status)
}

pub fn midi_note_status_intensity_indicator(
    note: u32,
    status: &bool,
    intensity: u32,
) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| midi_note_status_intensity_indicator_ui(ui, note, status, intensity)
}

pub fn midi_two_value_indicator(note: u32, intensity: u32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| midi_two_value_indicator_ui(ui, note, intensity)
}

pub fn midi_id_value_indicator(note: u32, intensity: u32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| midi_id_value_indicator_ui(ui, note, intensity)
}

pub fn midi_id_double_precision_value_indicator(note: u32, intensity: u32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| midi_id_double_precision_value_indicator_ui(ui, note, intensity)
}
