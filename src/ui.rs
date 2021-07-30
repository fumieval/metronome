use iced_baseview::{
    executor, Color, Command, Container, Element, Length,
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    Point, Rectangle, Vector,
};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    TimeChanged(chrono::DateTime<chrono::Local>),
}

pub struct MyPluginUI {
    now: chrono::DateTime<chrono::Local>,
    clock: Cache,
}

impl iced_baseview::Application for MyPluginUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                now: chrono::Local::now(),
                clock: Default::default(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TimeChanged(now) => {
                if now != self.now {
                    self.now = now;
                    self.clock.clear();
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let canvas = Canvas::new(self)
            .width(Length::Units(400))
            .height(Length::Units(400));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn renderer_settings() -> iced_baseview::renderer::Settings {
        iced_baseview::renderer::Settings {
            default_font: None,
            default_text_size: 20,
            antialiasing: Some(iced_baseview::renderer::Antialiasing::MSAAx8),
            ..iced_baseview::renderer::Settings::default()
        }
    }
}

// taken from https://github.com/hecrj/iced/blob/master/examples/clock/src/main.rs
impl canvas::Program<Message> for MyPluginUI {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        use chrono::Timelike;

        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));

            let short_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand =
                Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let thin_stroke = Stroke {
                width: radius / 100.0,
                color: Color::WHITE,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let wide_stroke = Stroke {
                width: thin_stroke.width * 3.0,
                ..thin_stroke
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.hour(), 12));
                frame.stroke(&short_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.second(), 60));
                frame.stroke(&long_hand, thin_stroke);
            })
        });

        vec![clock]
    }
}

fn hand_rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    2.0 * std::f32::consts::PI * turns
}