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
                        Fieldset {
                            legend: "Email",
                            legend_class: "mt-4",
                            help_text: "The email address of the person you wish to invite",
                            Input {
                                input_type: InputType::Email,
                                required: true,
                                name: "email"
                            }
                        }
                        Fieldset {
                            legend: "First Name",
                            legend_class: "mt-4",
                            help_text: "The first name of the person you wish to invite",
                            Input {
                                input_type: InputType::Text,
                                required: true,
                                name: "first_name"
                            }
                        }
                        Fieldset {
                            legend: "Last Name",
                            legend_class: "mt-4",
                            help_text: "The last name of the person you wish to invite",
                            Input {
                                input_type: InputType::Text,
                                required: true,
                                name: "last_name"
                            }
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
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            button_size: ButtonSize::Small,
                            "Cancel"
                        }
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
