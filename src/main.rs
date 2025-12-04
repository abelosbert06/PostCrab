use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmApp, SimpleComponent};

use crate::net::{ContentType, send_request};
mod misc;
mod net;

pub struct Model {
    textbox_text: String,
    request_type: net::RequestType,
    content_type: net::ContentType,
    error_message: Option<String>,
    input_enabled: bool,
    message_body_buff: sourceview5::Buffer,
    syntax_language: String,
    response_body_buff: sourceview5::Buffer,
}

#[derive(Debug)]
pub enum States {
    UpdateText(String),
    SendRequest,
    RequestTypeSelected(u32),
    ContentTypeSelected(u32),
    RequestFinished(Result<String, String>),
    BodySyntaxChanged(ContentType),
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Input = States;
    type Output = ();
    type Init = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        //syntax-highlighting init
        let inp_sourceview_buff = sourceview5::Buffer::new(None);
        crate::misc::syntax_highlighter(&inp_sourceview_buff, "json");

        let resp_sourceview_buff = sourceview5::Buffer::new(None);
        crate::misc::syntax_highlighter(&resp_sourceview_buff, "json");
        let model = Model {
            textbox_text: String::new(),
            request_type: net::RequestType::Get,
            content_type: net::ContentType::Json,
            error_message: None,
            input_enabled: false,
            message_body_buff: inp_sourceview_buff,
            response_body_buff: resp_sourceview_buff,
            syntax_language: "json".to_string(),
        };

        //workaround for checkbutton issue
        let first_button = gtk::CheckButton::builder()
            .label("Json")
            .active(true)
            .build();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Model>) {
        match msg {
            States::UpdateText(text) => {
                self.textbox_text = text;
            }

            States::RequestFinished(result) => match result {
                Ok(res_body) => {
                    crate::misc::syntax_highlighter(
                        &self.response_body_buff,
                        crate::misc::auto_detect_lang(&res_body),
                    );
                    self.response_body_buff.set_text(&res_body);
                    self.error_message = None;
                }

                Err(res_err) => {
                    self.error_message = Some(res_err);
                }
            },

            States::SendRequest => {
                let clean_address = self.textbox_text.trim().to_string();
                let req_type = self.request_type.clone();
                let req_content_type = self.content_type.clone();

                if clean_address.is_empty() {
                    self.error_message = Some("URL must not be empty.".to_string())
                } else {
                    self.error_message = None;

                    let start_iter = self.message_body_buff.start_iter();
                    let end_iter = self.message_body_buff.end_iter();
                    let raw_text = self.message_body_buff.text(&start_iter, &end_iter, true);

                    let message_body_option: Option<String> = if raw_text.is_empty() {
                        None
                    } else {
                        Some(raw_text.to_string())
                    };

                    let input_sender = sender.input_sender().clone();
                    relm4::spawn(async move {
                        let result = send_request(
                            &clean_address,
                            &req_type,
                            &message_body_option,
                            &req_content_type,
                        )
                        .await;

                        let res_message = match result {
                            Ok(msg) => Ok(msg),
                            Err(e) => Err(e.to_string()),
                        };

                        let _ = input_sender.send(States::RequestFinished(res_message));
                    });
                }
            }

            States::ContentTypeSelected(index) => {
                self.content_type = match index {
                    0 => net::ContentType::Json,
                    1 => net::ContentType::Text,
                    2 => net::ContentType::Xml,
                    3 => net::ContentType::Form,
                    _ => net::ContentType::Json,
                }
            }

            States::RequestTypeSelected(index) => {
                self.request_type = match index {
                    0 => net::RequestType::Get,
                    1 => net::RequestType::Post,
                    2 => net::RequestType::Delete,
                    3 => net::RequestType::Put,
                    4 => net::RequestType::Patch,
                    _ => net::RequestType::Get,
                };

                self.input_enabled = match index {
                    0 => false,
                    2 => false,
                    _ => true,
                }
            }

            States::BodySyntaxChanged(content_type) => {
                self.syntax_language = match content_type {
                    ContentType::Json => "json".to_string(),
                    ContentType::Text => "text".to_string(),
                    ContentType::Form => "ini".to_string(),
                    ContentType::Xml => "xml".to_string(),
                };

                crate::misc::syntax_highlighter(&self.message_body_buff, &self.syntax_language);
            }
        }
    }

    view! {
        gtk::Window{
            set_title: Some("PostCrab"),
            set_default_size: (1200, 900),

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 40,
                set_margin_all: 30,


                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 20,

                    gtk::DropDown{
                        set_model: Some(&gtk::StringList::new(&[
                            "Get",
                            "Post",
                            "Delete",
                            "Put",
                            "Patch"
                        ])),

                        set_selected: 0,

                        connect_selected_notify[sender] => move |dropdown| {
                            sender.input(States::RequestTypeSelected(dropdown.selected()));
                        }
                    },


                    gtk::Entry{
                        set_placeholder_text: Some("Enter address..."),
                        set_hexpand: true,

                        //update text
                        connect_changed[sender] => move |entry| {
                            sender.input(States::UpdateText(entry.text().to_string()));
                        }
                    },

                    gtk::Button {
                        set_label: "Send",
                        add_css_class: "send-button",
                        set_width_request: 100,
                        connect_clicked => States::SendRequest,
                    },
                },

                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 120,


                    #[local_ref]
                    first_button -> gtk::CheckButton{
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(States::ContentTypeSelected(0));
                                sender.input(States::BodySyntaxChanged(ContentType::Json));
                            }
                        },
                    },

                    gtk::CheckButton{
                        set_label: Some("Text"),
                        set_group: Some(&first_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(States::ContentTypeSelected(1));
                                sender.input(States::BodySyntaxChanged(ContentType::Text));
                            }
                        },
                    },

                    gtk::CheckButton{
                        set_label: Some("Xml"),
                        set_group: Some(&first_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(States::ContentTypeSelected(2));
                                sender.input(States::BodySyntaxChanged(ContentType::Xml));
                            }
                        },
                    },

                    gtk::CheckButton{
                        set_label: Some("Form"),
                        set_group: Some(&first_button),

                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(States::ContentTypeSelected(3));
                                sender.input(States::BodySyntaxChanged(ContentType::Form));
                            }
                        },
                    }
                },

                gtk::Label {

                    #[watch]
                    set_visible: model.error_message.is_some(),

                   #[watch]
                   set_label: model.error_message.as_deref().unwrap_or(""),

                   add_css_class: "error-message",

                },


                gtk::ScrolledWindow{

                    #[watch]
                    set_visible: model.input_enabled,

                    add_css_class: "input-box",

                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &sourceview5::View {

                        //set_placeholder_text: Some("Request content"),
                        set_monospace: true,
                        set_buffer: Some(&model.message_body_buff),
                    }
                },

                gtk::ScrolledWindow {
                    #[watch]
                    set_visible: model.error_message.is_none(),

                    add_css_class: "output-box",

                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &sourceview5::View {
                        set_cursor_visible: false,
                        set_editable: false,
                        set_monospace: true,

                        #[watch]
                        set_buffer: Some(&model.response_body_buff),

                    }

                },
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("post.crab");
    relm4::set_global_css(
        "
        .error-message {
            color: #e62d42;
        }
        .send-button {
            background-color: #3584e4;
        }

        .input-box{
            border-radius: 12px;
            padding: 16px;

            background-color: #1d1d20;
        }

        .indput-box sourceview {
            background-color: transparent;
            color: #fcfcfc;
            font-family: monospace;
            font-size: 14px;
        }

        .output-box{
            border-radius: 12px;
            padding: 16px;

            background-color: #1d1d20;
        }

        .output-box sourceview {
            background-color: transparent;
            color: #fcfcfc;
            font-family: monospace;
            font-size: 14px;
        }
    ",
    );
    app.run::<Model>(());
}
