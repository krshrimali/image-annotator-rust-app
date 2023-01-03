use std::path::PathBuf;

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable},
    widget::{container, image, text, Button, Column},
    Element, Length, Renderer, Sandbox, Theme,
};
use iced_native::Layout;

use self::render_image::{init_json_obj, AnnotatedStore, Message, Step, StepMessage};
// use notify_rust::Notification;
// use serde::{Deserialize, Serialize};
// use serde_json::json;

#[path = "render_image.rs"]
mod render_image;

#[derive(Default, Debug)]
pub struct Steps {
    steps: Vec<render_image::Step>,
    folder_path: String,
    curr_idx: usize,
    all_images: Vec<PathBuf>,
    correct_items: Vec<bool>,
    json_obj: AnnotatedStore,
    current: usize,
}

#[derive(Default, Debug)]
pub struct FolderVisualizer {
    steps: Steps,
}

fn get_all_images(folder_path: &String) -> Vec<PathBuf> {
    // TODO: Handle dir validation here
    let paths = std::fs::read_dir(folder_path).unwrap();
    let mut output: Vec<PathBuf> = vec![];
    for path in paths {
        output.push(path.unwrap().path().as_path().to_owned());
    }
    output
}

impl Sandbox for FolderVisualizer {
    type Message = render_image::Message;

    fn new() -> FolderVisualizer {
        let folder_path: String = "sample_folder".into();
        let all_images = get_all_images(&folder_path);
        let json_obj: AnnotatedStore = init_json_obj(all_images.len());
        let mut steps_obj = Steps::new(folder_path, 0, all_images.clone(), vec![], json_obj);
        steps_obj.correct_items = vec![false; all_images.len()];
        let folder_obj = FolderVisualizer {
            // folder_path,
            // curr_idx: 0,
            // all_images,
            // correct_items: vec![],
            steps: steps_obj,
            // json_obj,
        };
        folder_obj
    }

    fn title(&self) -> String {
        format!("Image {0}", self.steps.curr_idx)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer> {
        let FolderVisualizer { steps, .. } = self;
        let mut controls = row![];

        if steps.has_previous() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button("Next")
                    .on_press(Message::NextPressed)
                    .style(theme::Button::Primary),
            );
        }

        let content: Element<_> = column![steps.view().map(Message::StepMessage), controls,]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        let scrollable = scrollable(container(content).width(Length::Fill).center_x());
        container(scrollable).height(Length::Fill).center_y().into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    // fn theme(&self) -> Theme {
    //     self.theme.clone()
    // }
}

impl Steps {
    pub fn new(
        folder_path: String,
        curr_idx: usize,
        all_images: Vec<PathBuf>,
        correct_items: Vec<bool>,
        json_obj: AnnotatedStore,
    ) -> Steps {
        Steps {
            steps: vec![Step::WelcomeWithFolderChoose, Step::Images, Step::End],
            folder_path,
            curr_idx,
            all_images,
            correct_items,
            json_obj,
            current: 0,
        }
    }

    pub fn update(&mut self, msg: StepMessage) {
        let (new_idx, new_indices, new_values, new_correct_items) = self.steps[self.current].update(msg, &mut self.curr_idx, self.json_obj.indices.clone(), self.json_obj.values.clone(), &mut self.correct_items);
        self.curr_idx = new_idx;
        self.json_obj.indices = new_indices;
        self.json_obj.values = new_values;
        self.correct_items = new_correct_items;
    }

    pub fn view(&self) -> Element<StepMessage> {
        self.steps[self.current].view(self)
    }

    pub fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    pub fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    pub fn has_previous(&self) -> bool {
        self.current > 0
    }

    pub fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    pub fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}
