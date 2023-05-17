use std::{fs, io::Read};

use tonic::{transport::Channel, Request};
use tuihub_protos::librarian::sephirah::v1::{
    librarian_sephirah_service_client::LibrarianSephirahServiceClient, FileMetadata, FileType,
    SimpleUploadFileRequest, UploadImageRequest,
};

use crate::Result;

pub struct Client {
    rpc: LibrarianSephirahServiceClient<Channel>,
    token: String,
}

pub async fn init(host: String, port: String, token: String) -> Result<Client> {
    let client =
        LibrarianSephirahServiceClient::connect(format!("https://{}:{}", host, port)).await?;

    Ok(Client { rpc: client, token })
}

impl Client {
    pub async fn upload_image(&mut self, file_name: String, size: i64) -> Result<String> {
        let mut request = Request::new(UploadImageRequest {
            file_metadata: Some(FileMetadata {
                id: None,
                name: file_name,
                size,
                r#type: FileType::ChesedImage.into(),
                sha256: Default::default(),
            }),
            name: "".to_string(),
            description: "".to_string(),
        });
        request
            .metadata_mut()
            .insert("authorization", format!("bearer {}", self.token).parse()?);
        let response = self.rpc.upload_image(request).await?;
        Ok(response.into_inner().upload_token)
    }

    pub async fn upload_file(&mut self, mut reader: fs::File, token: String) -> Result<()> {
        let mut request = Request::new(async_stream::stream! {
            let mut buffer = [0u8; 32*1024];
            loop {
                let n = match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {n},
                    _ => panic!("")
                };
                let note = SimpleUploadFileRequest{
                    data: bytes::Bytes::from(Vec::from(&buffer[..n])),
                };
                yield note;
            }
        });
        request
            .metadata_mut()
            .insert("authorization", format!("bearer {}", token).parse()?);

        let response = self.rpc.simple_upload_file(request).await?;
        let mut inbound = response.into_inner();

        while let Some(note) = inbound.message().await? {
            println!("NOTE = {:?}", note);
        }
        Ok(())
    }
}
