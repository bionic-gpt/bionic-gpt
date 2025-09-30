#![allow(non_snake_case)]

use daisy_rsx::*;
use db::Licence;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct McpUrlModalProps {
    pub id_prefix: String,
    pub connection_id: i32,
    pub external_id: String,
    pub mcp_slug: Option<String>,
    pub connection_label: String,
}

pub fn McpUrlModal(props: McpUrlModalProps) -> Element {
    let McpUrlModalProps {
        id_prefix,
        connection_id,
        external_id,
        mcp_slug,
        connection_label,
    } = props;

    let Some(slug) = mcp_slug else {
        return rsx! {};
    };

    let modal_id = format!("{}{}", id_prefix, connection_id);
    let hostname_root = Licence::global().hostname_url.trim_end_matches('/');
    let path = format!("/v1/mcp/{}/{}", slug, external_id);
    let mcp_url = if hostname_root.is_empty() {
        path.clone()
    } else {
        format!("{}{}", hostname_root, path)
    };

    let claude_config = format!(
        "\"{}\": {{\n      \"transport\": \"http\",\n      \"url\": \"{}\"\n    }}",
        slug, mcp_url
    );

    rsx! {
        Button {
            popover_target: modal_id.clone(),
            button_style: ButtonStyle::Outline,
            button_scheme: ButtonScheme::Primary,
            button_size: ButtonSize::Small,
            "View MCP URL"
        }
        Modal {
            trigger_id: &modal_id,
            ModalBody {
                h3 {
                    class: "font-bold text-lg mb-2",
                    "Machine Connection Protocol URL"
                }
                p {
                    class: "text-sm text-base-content/70 mb-3",
                    "Use this URL to connect your MCP client to this {connection_label}."
                }
                textarea {
                    class: "textarea textarea-bordered w-full text-sm font-mono bg-base-200",
                    readonly: true,
                    rows: "3",
                    "{mcp_url}"
                }
                div {
                    class: "space-y-2 mt-4",
                    span {
                        class: "text-sm font-semibold text-base-content/80",
                        "Claude Config"
                    }
                    textarea {
                        class: "textarea textarea-bordered w-full text-sm font-mono bg-base-200",
                        readonly: true,
                        rows: "4",
                        "{claude_config}"
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Primary,
                        button_size: ButtonSize::Small,
                        "Close"
                    }
                }
            }
        }
    }
}
