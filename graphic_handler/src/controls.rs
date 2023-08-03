use iced_glow::Renderer;
use iced_glutin::widget::{Button, Column, Row, Text};
use iced_glutin::widget::{Checkbox, Slider};
use iced_glutin::{Alignment, Color, Command, Element, Length, Program};
use nalgebra_glm::{vec3, Vec3};

use crate::graphic_config::GraphicConfig;

#[derive(Debug, Clone)]
pub struct Controls {
    pub refresh: u8,
    pub command_panel: bool,
    pub camera_position: Vec3,
    pub scene_scale_lock: bool,
    pub scene_scale: Vec3,
    pub scene_position: Vec3,
    pub scene_rotation: Vec3,
    pub fps: u128,
    pub filter_id: i32,
    pub animation_id: i32,
    pub mode_id: i32,
    pub debug: i32,
    // pub music_player
}

#[derive(Debug, Clone)]
pub enum Message {
    CameraPositionChanged(Vec3),
    SceneScaleChanged(Vec3),
    ScenePositionChanged(Vec3),
    SceneRotationChanged(Vec3),
    FpsChanged(u128),
    FilterChanged(i32),
    AnimationChanged(i32),
    ModeChanged(i32),
    DebugChanged(i32),
    CommandPanelChanged(bool),
    SceneScaleLockChanged(bool),
}

impl Controls {
    pub fn new(config: &GraphicConfig) -> Controls {
        Controls {
            refresh: 0,
            camera_position: vec3(0.0, 0.0, 0.0),
            scene_scale: vec3(0.5, 0.5, 0.5),
            scene_position: vec3(0.0, 0.0, 0.0),
            scene_rotation: vec3(0.0, 1.0, 0.0),
            fps: config.fps,
            filter_id: 0,
            animation_id: 0,
            mode_id: 0,
            debug: 1,
            command_panel: false,
            scene_scale_lock: false,
        }
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CameraPositionChanged(position) => {
                self.camera_position = position;
            }
            Message::SceneScaleChanged(scale) => {
                self.scene_scale = scale;
            }
            Message::ScenePositionChanged(position) => {
                self.scene_position = position;
            }
            Message::SceneRotationChanged(rotation) => {
                self.scene_rotation = rotation;
            }
            Message::FpsChanged(fps) => {
                self.fps = fps;
            }
            Message::FilterChanged(filter) => {
                self.filter_id = filter;
            }
            Message::AnimationChanged(animation) => {
                self.animation_id = animation;
            }
            Message::ModeChanged(mode) => {
                self.mode_id = mode;
            }
            Message::DebugChanged(debug) => {
                self.debug = debug;
            }
            Message::CommandPanelChanged(command_panel) => {
                self.command_panel = command_panel;
            }
            Message::SceneScaleLockChanged(scene_scale_lock) => {
                self.scene_scale_lock = scene_scale_lock;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer> {
        let scene_scale_lock = self.scene_scale_lock;
        let scene_scale = self.scene_scale;
        let scene_scale_sliders = Row::<Message, Renderer>::new()
            .width(500)
            .spacing(20)
            .push(
                Button::new(if scene_scale_lock {
                    "Lock scale"
                } else {
                    "Unlock scale"
                })
                .on_press(Message::SceneScaleLockChanged(!scene_scale_lock)),
            )
            .push(
                Slider::new(0.01..=2.0, scene_scale.x, move |x| {
                    if !scene_scale_lock {
                        Message::SceneScaleChanged(vec3(x, scene_scale.y, scene_scale.z))
                    } else {
                        Message::SceneScaleChanged(vec3(x, x, x))
                    }
                })
                .step(0.01),
            )
            .push(
                Slider::new(0.01..=2.0, scene_scale.y, move |y| {
                    if !scene_scale_lock {
                        Message::SceneScaleChanged(vec3(scene_scale.x, y, scene_scale.z))
                    } else {
                        Message::SceneScaleChanged(vec3(y, y, y))
                    }
                })
                .step(0.01),
            )
            .push(
                Slider::new(0.01..=2.0, scene_scale.z, move |z| {
                    if !scene_scale_lock {
                        Message::SceneScaleChanged(vec3(scene_scale.x, scene_scale.y, z))
                    } else {
                        Message::SceneScaleChanged(vec3(z, z, z))
                    }
                })
                .step(0.01),
            );

        let scene_position = self.scene_position;
        let scene_position_sliders = Row::<Message, Renderer>::new()
            .width(500)
            .spacing(20)
            .push(
                Slider::new(-50.0..=50.0, scene_position.x, move |x| {
                    Message::ScenePositionChanged(vec3(x, scene_position.y, scene_position.z))
                })
                .step(0.01),
            )
            .push(
                Slider::new(-50.0..=50.0, scene_position.y, move |y| {
                    Message::ScenePositionChanged(vec3(scene_position.x, y, scene_position.z))
                })
                .step(0.01),
            )
            .push(
                Slider::new(-50.0..=50.0, scene_position.z, move |z| {
                    Message::ScenePositionChanged(vec3(scene_position.x, scene_position.y, z))
                })
                .step(0.01),
            );

        let scene_rotation = self.scene_rotation;
        let scene_rotation_sliders = Row::<Message, Renderer>::new()
            .width(500)
            .spacing(20)
            .push(
                Slider::new(-10.0..=10.0, scene_rotation.x, move |x| {
                    Message::SceneRotationChanged(vec3(x, scene_rotation.y, scene_rotation.z))
                })
                .step(0.01),
            )
            .push(
                Slider::new(-10.0..=10.0, scene_rotation.y, move |y| {
                    Message::SceneRotationChanged(vec3(scene_rotation.x, y, scene_rotation.z))
                })
                .step(0.01),
            )
            .push(
                Slider::new(-10.0..=10.0, scene_rotation.z, move |z| {
                    Message::SceneRotationChanged(vec3(scene_rotation.x, scene_rotation.y, z))
                })
                .step(0.01),
            );

        let debug = self.debug;
        let debug_checkbox =
            Row::<Message, Renderer>::new()
                .width(50)
                .spacing(20)
                .push(Checkbox::new(
                    "",
                    if debug == 1 { true } else { false },
                    move |_| Message::DebugChanged(if debug == 1 { 0 } else { 1 }),
                ));

        let fps = self.fps as i32;
        let fps_slider = Row::<Message, Renderer>::new()
            .width(350)
            .spacing(20)
            .push(Slider::new(1..=120, fps, move |x| Message::FpsChanged(x as u128)).step(1));

        let command_panel = self.command_panel;
        let command_panel_button = Row::<Message, Renderer>::new().width(500).spacing(20).push(
            Button::new(if command_panel {
                "Show command panel"
            } else {
                "Hide command panel"
            })
            .on_press(Message::CommandPanelChanged(!command_panel)),
        );
        let mut c = Column::new().push(command_panel_button);
        if command_panel {
            c = c
                .padding(10)
                .spacing(10)
                .push(Text::new("Scene position").style(Color::WHITE))
                .push(scene_position_sliders)
                .push(
                    Text::new(format!("{scene_position:?}"))
                        .size(14)
                        .style(Color::WHITE),
                )
                .push(Text::new("Scene Scale").style(Color::WHITE))
                .push(scene_scale_sliders)
                .push(
                    Text::new(format!("{scene_scale:?}"))
                        .size(14)
                        .style(Color::WHITE),
                )
                .push(Text::new("Scene Rotation").style(Color::WHITE))
                .push(scene_rotation_sliders)
                .push(
                    Text::new(format!("{scene_rotation:?}"))
                        .size(14)
                        .style(Color::WHITE),
                )
                .push(
                    Row::new()
                        .push(Text::new("Debug").style(Color::WHITE))
                        .spacing(10)
                        .push(debug_checkbox)
                        .push(
                            Text::new(format!("FPS: {fps:?}"))
                                .size(14)
                                .style(Color::WHITE),
                        )
                        .spacing(10)
                        .push(fps_slider),
                );
        }
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .push(c),
            )
            .into()
    }
}
