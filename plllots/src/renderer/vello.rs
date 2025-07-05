use crate::chart::Chart;
use kurbo::{Affine, BezPath, Circle, Line, Point, Rect, Shape};
use parley::{Alignment, AlignmentOptions};
use parley::{FontContext, LayoutContext, PositionedLayoutItem, style::StyleProperty};
use peniko::{Brush, Fill};
use vello::Scene;

pub struct VelloRenderer {
    font_cx: FontContext,
    layout_cx: LayoutContext,
}

impl Default for VelloRenderer {
    fn default() -> Self {
        Self::new()
    }
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
            &Brush::Solid(chart.theme.background),
            None,
            &Rect::new(0.0, 0.0, chart.size.width, chart.size.height),
        );

        let primitives = chart.generate_primitives();
        for primitive in primitives {
            primitive.append_vello(scene, self);
        }
    }
}

pub trait AppendVello {
    fn append_vello(&self, scene: &mut Scene, vello_render: &mut VelloRenderer);
}

impl AppendVello for crate::primitives::Line<'_> {
    fn append_vello(&self, scene: &mut Scene, _vello_render: &mut VelloRenderer) {
        scene.stroke(
            self.stroke,
            Affine::IDENTITY,
            self.stroke_color,
            None,
            &Line::new(self.coords.0, self.coords.1),
        );
    }
}

impl AppendVello for crate::primitives::Text<'_> {
    fn append_vello(&self, scene: &mut Scene, vello_render: &mut VelloRenderer) {
        let mut layout_builder =
            vello_render
                .layout_cx
                .ranged_builder(&mut vello_render.font_cx, &self.text, 1.0);
        layout_builder.push_default(StyleProperty::FontSize(self.font_size as f32));
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

        let transform = match self.rotation {
            Some(r) => transform.then_rotate_about(
                r,
                Point {
                    x: self.coord.x - text_x_offset as f64 / 1.0 + text_width as f64 / 2.0,
                    y: self.coord.y,
                },
            ),
            None => transform,
        };

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

impl AppendVello for crate::primitives::Path<'_> {
    fn append_vello(&self, scene: &mut Scene, _vello_render: &mut VelloRenderer) {
        let mut path = BezPath::new();

        for (index, point) in self.coords.iter().enumerate() {
            if index == 0 {
                path.move_to(*point);
            } else {
                path.line_to(*point);
            }
        }

        scene.stroke(
            self.stroke,
            Affine::IDENTITY,
            self.stroke_color,
            None,
            &path,
        );
    }
}

impl AppendVello for crate::primitives::Circle<'_> {
    fn append_vello(&self, scene: &mut Scene, _vello_render: &mut VelloRenderer) {
        let circle = Circle::new(self.coord, self.radius);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            self.fill_color,
            None,
            &circle,
        );
        scene.stroke(
            self.stroke,
            Affine::IDENTITY,
            self.stroke_color,
            None,
            &circle,
        );
    }
}

impl AppendVello for crate::primitives::MultiCircle<'_> {
    fn append_vello(&self, scene: &mut Scene, _vello_render: &mut VelloRenderer) {
        let circle = Circle::new((0.0, 0.0), self.radius).into_path(0.1);
        for coord in &self.coords {
            let transform = Affine::translate((coord.x, coord.y));
            scene.fill(Fill::NonZero, transform, self.fill_color, None, &circle);
            scene.stroke(self.stroke, transform, self.stroke_color, None, &circle);
        }
    }
}
