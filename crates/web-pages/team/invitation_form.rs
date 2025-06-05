#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn InvitationForm(submit_action: String) -> Element {
    rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "{submit_action}",
            Modal {
                trigger_id: "create-invite-form",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Invite people into your team."
                    }
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Email,
                            help_text: "The email address of the person you wish to invite",
                            required: true,
                            label: "Email",
                            label_class: "mt-4",
                            name: "email"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "The first name of the person you wish to invite",
                            required: true,
                            label: "First Name",
                            label_class: "mt-4",
                            name: "first_name"
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "The last name of the person you wish to invite",
                            required: true,
                            label: "Last Name",
                            label_class: "mt-4",
                            name: "last_name"
                        }
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mt-4 flex flex-col items-start",
                            label {
                                input {
                                    "type": "checkbox",
                                    name: "admin"
                                }
                                strong {
                                    class: "ml-2",
                                    "Invite as Team Manager"
                                }
                            }
                            p {
                                class: "note",
                                "Team Managers can invite new team members"
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Send Invitation"
                        }
                    }
                }
            }
        }
    }
}
