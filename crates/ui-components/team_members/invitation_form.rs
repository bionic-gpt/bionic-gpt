#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn InvitationForm(cx: Scope, submit_action: String) -> Element {
    cx.render(rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "{submit_action}",
            Drawer {
                label: "Invite people into your team.",
                trigger_id: "create-invite-form",
                DrawerBody {
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
                                    "Invite as Team Administrator"
                                }
                            }
                            p {
                                class: "note",
                                "Team Administrators can invite new team members"
                            }
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Send Invitation"
                    }
                }
            }
        }
    })
}
