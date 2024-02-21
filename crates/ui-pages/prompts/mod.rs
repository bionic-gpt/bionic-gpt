pub mod dataset_connection;
pub mod delete;
pub mod form;
pub mod index;
pub mod visibility;

use db::DatasetConnection;
pub use index::index;

pub fn dataset_connection_to_string(connection: DatasetConnection) -> String {
    match connection {
        DatasetConnection::All => "All".to_string(),
        DatasetConnection::Selected => "Use the datasets selected below".to_string(),
        _ => "Don't use any datasets".to_string(),
    }
}

pub fn string_to_dataset_connection(connection: &str) -> DatasetConnection {
    match connection {
        "All" => DatasetConnection::All,
        "Use the datasets selected below" => DatasetConnection::Selected,
        _ => DatasetConnection::None,
    }
}
