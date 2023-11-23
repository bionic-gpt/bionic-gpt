#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn Upload(cx: Scope, upload_action: String) -> Element {
    cx.render(rsx!(
        form {
            action: "{upload_action}",
            method: "post",
            enctype: "multipart/form-data",
            Drawer {
                label: "Upload a file into this dataset",
                trigger_id: "upload-form",
                DrawerBody {

                    input {
                        "type": "file",
                        name: "payload"
                    }

                    Alert {
                        class: "mt-4",
                        alert_color: AlertColor::Warn,
                        "Max file size 50MB"
                    }

                    Alert {
                        class: "mt-4",
                        alert_color: AlertColor::Default,
                        h5 {
                            "Supported File Types"
                        }

                        ul {
                            class: "pl-3 mt-4",
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
                }

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        disabled_text: "Document uploading, this may take some time",
                        "Upload File"
                    }
                }
            }
        }
    ))
}
