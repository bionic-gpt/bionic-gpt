pub mod dataset_connection;
pub mod form;
pub mod index;
pub mod visibility;

use db::DatasetConnection;
pub use index::index;

pub fn dataset_connection_to_string(connection: DatasetConnection) -> String {
    match connection {
        DatasetConnection::All => "All".to_string(),
        DatasetConnection::Selected => "Selected".to_string(),
        _ => "None".to_string(),
    }
}

pub fn string_to_dataset_connection(connection: &str) -> DatasetConnection {
    match connection {
        "All" => DatasetConnection::All,
        "Selected" => DatasetConnection::Selected,
        _ => DatasetConnection::None,
    }
}
