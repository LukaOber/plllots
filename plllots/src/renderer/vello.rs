use crate::chart::{Chart, ChartPlotHelper};
use crate::component::{AxisData, AxisHelper};
use kurbo::{Affine, BezPath, Circle, Line, PathEl, Point, Rect, Size, Stroke};
use parley::{Alignment, AlignmentOptions};
use parley::{
    FontContext, Layout, LayoutContext, PositionedLayoutItem,
    style::{FontFamily, FontStack, FontStyle, FontWeight, StyleProperty},
};
use peniko::{Brush, Color, Fill};
use vello::{AaConfig, RenderParams, Renderer, Scene};

/// Vello renderer for charts.
pub struct VelloRenderer {
    font_cx: FontContext,
    layout_cx: LayoutContext<Brush>,
}

impl VelloRenderer {
    /// Create a new Vello renderer.
    pub fn new() -> Self {
        Self {
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    /// Render a chart to a Vello scene.
    pub fn render_to_scene(&mut self, chart: &Chart, scene: &mut Scene) {
        let mut helper = ChartPlotHelper {
            plot_size: chart.size,
            margins: chart.margins,
            offsets: crate::element::Offsets::from_margin(&chart.size, &chart.margins),
            y_axis: None,
            x_axis: None,
        };

        // Calculate axis helpers
        self.calculate_axis_helpers(&mut helper, &chart.x_axis.data, &chart.y_axis.data);

        // Clear background
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(Color::WHITE),
            None,
            &Rect::new(0.0, 0.0, chart.size.width, chart.size.height),
        );

        // Render axes
        self.render_x_axis(scene, &helper, &chart.x_axis.data);
        self.render_y_axis(scene, &helper, &chart.y_axis.data);

        // Render line series
        self.render_line_series(scene, &helper, &chart.x_axis.data, &chart.y_axis.data);
    }

    fn calculate_axis_helpers(
        &self,
        helper: &mut ChartPlotHelper,
        x_data: &AxisData,
        y_data: &AxisData,
    ) {
        match x_data {
            AxisData::Category(items) => {
                helper.x_axis = Some(AxisHelper::Category(crate::component::AxisCategoryHelper {
                    amount: items.len(),
                }));
            }
            AxisData::Values(_) => todo!("Values X-axis not implemented yet"),
        }

        match y_data {
            AxisData::Values(items) => {
                let (min, max, step_size) = crate::utils::calculate_axis_ticks(items);
                helper.y_axis = Some(AxisHelper::Values(crate::component::AxisValuesHelper {
                    min,
                    max,
                    step_size,
                }));
            }
            AxisData::Category(_) => todo!("Category Y-axis not implemented yet"),
        }
    }

    fn render_x_axis(&mut self, scene: &mut Scene, helper: &ChartPlotHelper, x_data: &AxisData) {
        if let AxisData::Category(items) = x_data {
            // Draw main axis line
            let line = Line::new(
                Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
                Point::new(helper.offsets.x_axis_end, helper.offsets.y_axis_end),
            );
            scene.stroke(
                &Stroke::new(1.0),
                Affine::IDENTITY,
                &Brush::Solid(Color::from_rgb8(110, 112, 121)),
                None,
                &line,
            );

            let label_spacing = helper.offsets.x_span / items.len() as f64;

            // Draw tick marks
            for label_index in 0..=items.len() {
                let x_pos = helper.offsets.x_axis_start + label_index as f64 * label_spacing;
                let tick_line = Line::new(
                    Point::new(x_pos, helper.offsets.y_axis_end),
                    Point::new(x_pos, helper.offsets.y_axis_end + 5.0),
                );
                scene.stroke(
                    &Stroke::new(1.0),
                    Affine::IDENTITY,
                    &Brush::Solid(Color::from_rgb8(110, 112, 121)),
                    None,
                    &tick_line,
                );
            }

            // Draw labels
            for (label_index, label) in items.iter().enumerate() {
                let mut layout_builder =
                    self.layout_cx.ranged_builder(&mut self.font_cx, label, 1.0);
                layout_builder.push_default(StyleProperty::FontSize(12.0));
                let mut layout = layout_builder.build(label);
                layout.break_all_lines(None);
                layout.align(None, Alignment::Middle, AlignmentOptions::default());

                let x_pos =
                    helper.offsets.x_axis_start + (label_index as f64 + 0.5) * label_spacing;
                let y_pos = helper.offsets.y_axis_end + 5.0;

                // Center the text horizontally
                let text_width = layout.width();
                let transform = Affine::translate((x_pos - text_width as f64 / 2.0, y_pos));

                for line in layout.lines() {
                    for item in line.items() {
                        if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                            let mut x = glyph_run.offset();
                            let y = glyph_run.baseline();
                            let run = glyph_run.run();
                            let font = run.font();
                            let font_size = run.font_size();
                            let synthesis = run.synthesis();
                            let glyph_xform = synthesis
                                .skew()
                                .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                            let coords = run.normalized_coords();

                            scene
                                .draw_glyphs(font)
                                .brush(&Brush::Solid(Color::from_rgb8(0x6E, 0x70, 0x79).into()))
                                .transform(transform)
                                .glyph_transform(glyph_xform)
                                .font_size(font_size)
                                .normalized_coords(coords)
                                .draw(
                                    Fill::NonZero,
                                    glyph_run.glyphs().map(|glyph| {
                                        let gx = x + glyph.x;
                                        let gy = y - glyph.y;
                                        x += glyph.advance;
                                        vello::Glyph {
                                            id: glyph.id as _,
                                            x: gx,
                                            y: gy,
                                        }
                                    }),
                                );
                        }
                    }
                }
            }
        }
    }

    fn render_y_axis(&mut self, scene: &mut Scene, helper: &ChartPlotHelper, y_data: &AxisData) {
        if let AxisData::Values(items) = y_data {
            if let Some(AxisHelper::Values(y_axis_helper)) = &helper.y_axis {
                let sub_tick_spacing =
                    helper.offsets.y_span / (y_axis_helper.max / y_axis_helper.step_size);

                // Draw grid lines
                for sub_tick_index in 1..((y_axis_helper.max / y_axis_helper.step_size) as i32 + 1)
                {
                    let sub_tick_height =
                        helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
                    let grid_line = Line::new(
                        Point::new(helper.offsets.x_axis_start, sub_tick_height),
                        Point::new(helper.offsets.x_axis_end, sub_tick_height),
                    );
                    scene.stroke(
                        &Stroke::new(1.0),
                        Affine::IDENTITY,
                        &Brush::Solid(Color::from_rgb8(224, 230, 241)),
                        None,
                        &grid_line,
                    );
                }

                // Draw labels
                let mut layout_builder = self.layout_cx.ranged_builder(&mut self.font_cx, "", 1.0);
                layout_builder.push_default(StyleProperty::FontSize(12.0));

                for sub_tick_index in 0..((y_axis_helper.max / y_axis_helper.step_size) as i32 + 1)
                {
                    let sub_tick_height =
                        helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
                    let label_text = format!(
                        "{}",
                        y_axis_helper.min + y_axis_helper.step_size * sub_tick_index as f64
                    );
                    let layout = layout_builder.build(label_text);

                    let text_width = layout.width();
                    let text_height = layout.height();
                    let transform = Affine::translate((
                        helper.offsets.x_axis_start - text_width as f64 - 8.0,
                        sub_tick_height - text_height as f64 / 2.0,
                    ));

                    for line in layout.lines() {
                        for item in line.items() {
                            if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                                scene
                                    .draw_glyphs(&glyph_run.run().font())
                                    .brush(&Brush::Solid(Color::from_rgb8(0xff, 0, 0).into()))
                                    .transform(transform);
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_line_series(
        &mut self,
        scene: &mut Scene,
        helper: &ChartPlotHelper,
        x_data: &AxisData,
        y_data: &AxisData,
    ) {
        if let (AxisData::Category(x_items), AxisData::Values(y_items)) = (x_data, y_data) {
            if let Some(AxisHelper::Values(y_axis_helper)) = &helper.y_axis {
                let mut path = BezPath::new();
                let mut points = Vec::new();

                for (index, (_x_item, y_item)) in x_items.iter().zip(y_items).enumerate() {
                    let y_pos = {
                        let percentage_height = y_item / y_axis_helper.max;
                        helper.offsets.y_axis_end - (percentage_height * helper.offsets.y_span)
                    };

                    let x_spacing = helper.offsets.x_span / x_items.len() as f64;
                    let x_pos = helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;

                    let point = Point::new(x_pos, y_pos);
                    points.push(point);

                    if index == 0 {
                        path.move_to(point);
                    } else {
                        path.line_to(point);
                    }
                }

                // Draw the line
                scene.stroke(
                    &Stroke::new(2.0),
                    Affine::IDENTITY,
                    &Brush::Solid(Color::from_rgb8(84, 112, 198)),
                    None,
                    &path,
                );

                // Draw the points
                for point in points {
                    let circle = Circle::new(point, 3.0);
                    scene.fill(
                        Fill::NonZero,
                        Affine::IDENTITY,
                        &Brush::Solid(Color::WHITE),
                        None,
                        &circle,
                    );
                    scene.stroke(
                        &Stroke::new(2.0),
                        Affine::IDENTITY,
                        &Brush::Solid(Color::from_rgb8(84, 112, 198)),
                        None,
                        &circle,
                    );
                }
            }
        }
    }
}

impl Default for VelloRenderer {
    fn default() -> Self {
        Self::new()
    }
}
