use crate::RenderSeries;
use crate::chart::{Chart, ChartPlotHelper};
use crate::component::{AxisHelper, CartesianAxis};
use kurbo::{Affine, BezPath, Circle, Line, PathEl, Point, Rect, Size, Stroke};
use parley::{Alignment, AlignmentOptions};
use parley::{
    FontContext, Layout, LayoutContext, PositionedLayoutItem,
    style::{FontFamily, FontStack, FontStyle, FontWeight, StyleProperty},
};
use peniko::{Brush, Color, Fill};
use vello::{AaConfig, RenderParams, Renderer, Scene};

pub struct VelloRenderer {
    font_cx: FontContext,
    layout_cx: LayoutContext,
}

impl VelloRenderer {
    pub fn new() -> Self {
        Self {
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    pub fn render_to_scene(&mut self, chart: &Chart, scene: &mut Scene) {
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(Color::WHITE),
            None,
            &Rect::new(0.0, 0.0, chart.size.width, chart.size.height),
        );

        let primitives = chart.generate_primitives();
        let mut helper = chart.create_plot_helper();
        // match &chart.x_axis.data {
        //     CartesianAxis::Category(items) => {
        //         helper.x_axis = Some(AxisHelper::Category(crate::component::AxisCategoryHelper {
        //             amount: items.len(),
        //         }));
        //     }
        //     CartesianAxis::Values(_) => todo!("Values X-axis not implemented yet"),
        // }

        // match &chart.y_axis.data {
        //     CartesianAxis::Values(items) => {
        //         let (min, max, step_size) = crate::utils::calculate_axis_ticks(&items);
        //         helper.y_axis = Some(AxisHelper::Values(crate::component::AxisValuesHelper {
        //             min,
        //             max,
        //             step_size,
        //         }));
        //     }
        //     CartesianAxis::Category(_) => todo!("Category Y-axis not implemented yet"),
        // }

        for primitive in primitives {
            primitive.append_vello(scene, self);
        }
        // println!("{:#?}", primitives);
        // match &chart.x_axis.data {
        //     CartesianAxis::Category(items) => {
        //         helper.x_axis = Some(AxisHelper::Category(crate::component::AxisCategoryHelper {
        //             amount: items.len(),
        //         }));
        //     }
        //     CartesianAxis::Values(_) => todo!("Values X-axis not implemented yet"),
        // }

        // match &chart.y_axis.data {
        //     CartesianAxis::Values(items) => {
        //         let (min, max, step_size) = crate::utils::calculate_axis_ticks(&items);
        //         helper.y_axis = Some(AxisHelper::Values(crate::component::AxisValuesHelper {
        //             min,
        //             max,
        //             step_size,
        //         }));
        //     }
        //     CartesianAxis::Category(_) => todo!("Category Y-axis not implemented yet"),
        // }

        // // Render series data
        // let line_series = LineSeries;
        // line_series.render_line_series(scene, &helper, &chart.x_axis.data, &chart.y_axis.data);
        // // Render axes
        // self.render_x_axis(scene, &helper, &chart.x_axis.data);
        // self.render_y_axis(scene, &helper, &chart.y_axis.data);

        // // Render line series
        // self.render_line_series(scene, &helper, &chart.x_axis.data, &chart.y_axis.data);
    }
}

pub trait AppendVello {
    fn append_vello(&self, scene: &mut Scene, vello_render: &mut VelloRenderer);
}

impl<'a> AppendVello for crate::primitives::Line<'a> {
    fn append_vello(&self, scene: &mut Scene, vello_render: &mut VelloRenderer) {
        scene.stroke(
            self.stroke,
            Affine::IDENTITY,
            self.stroke_color,
            None,
            &Line::new(self.coords.0, self.coords.1),
        );
    }
}

impl<'a> AppendVello for crate::primitives::Text<'a> {
    fn append_vello(&self, scene: &mut Scene, vello_render: &mut VelloRenderer) {
        let mut layout_builder =
            vello_render
                .layout_cx
                .ranged_builder(&mut vello_render.font_cx, &self.text, 1.0);
        layout_builder.push_default(StyleProperty::FontSize(12.0));
        let mut layout = layout_builder.build(&self.text);
        layout.break_all_lines(None);
        layout.align(None, self.text_anchor, AlignmentOptions::default());

        let text_width = layout.width();
        let text_height = layout.height();

        let text_x_offset = match self.text_anchor {
            Alignment::Start => 0.0,
            Alignment::End => text_width,
            Alignment::Left => 0.0,
            Alignment::Middle => text_width / 2.0,
            Alignment::Right => text_width,
            Alignment::Justified => text_width / 2.0,
        };

        let transform = Affine::translate((
            self.coord.x - text_x_offset as f64,
            self.coord.y - text_height as f64 / 2.0,
        ));
        // .then_rotate_about(
        //     3.14 / -2.0,
        //     Point {
        //         x: self.coord.x - text_x_offset as f64 / 1.0 + text_width as f64 / 2.0,
        //         y: self.coord.y,
        //     },
        // );
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
                        .brush(self.fill_color)
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
