#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Upload(upload_action: String) -> Element {
    rsx!(
        form {
            action: "{upload_action}",
            method: "post",
            enctype: "multipart/form-data",
            Modal {
                trigger_id: "upload-form",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Upload a file into this dataset"
                    }

                    input {
                        "type": "file",
                        name: "payload",
                        multiple: true
                    }

                    Alert {
                        class: "mt-4",
                        alert_color: AlertColor::Warn,
                        "Max file size 50MB"
                    }

                    Alert {
                        class: "mt-4 flex flex-col items-start",
                        alert_color: AlertColor::Default,
                        h5 {
                            "Supported File Types"
                        }

                        ul {
                            class: "mt-4",
                            li {
                                strong {"Plaintext "}
                                ".eml, .html, .json, .md, .msg, .rst, .rtf, .txt, .xml"
                            }
                            li {
                                strong {"Images "}
                                ".jpeg, .png"
                            }
                            li {
                                strong {"Documents "}
                                ".csv, .doc, .docx, .epub, .odt, .pdf, .ppt, .pptx, .tsv, .xlsx"
                            }
                        }
                    }

                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            disabled_text: "Document uploading, this may take some time",
                            "Upload File(s)"
                        }
                    }
                }
            }
        }
    )
}
