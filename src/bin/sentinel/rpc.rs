use std::{collections::HashMap, vec::IntoIter};

use log::debug;
use tonic::transport::Channel;
use tuihub_protos::librarian::{
    sephirah::v1::{
        librarian_sephirah_service_client::LibrarianSephirahServiceClient, ReportAppPackagesRequest,
    },
    v1::AppPackageBinary,
};

use crate::{scanner::ScanResult, Result};

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
    pub async fn report(mut self, list: Vec<ScanResult>) -> Result<()> {
        let mut request = tonic::Request::new(gen_report_req(list));
        request
            .metadata_mut()
            .insert("authorization", format!("bearer {}", self.token).parse()?);
        debug!("ReportReq: {:?}", request);
        let response = self.rpc.report_app_packages(request).await?;
        debug!("ReportResp: {:?}", response);
        Ok(())
    }
}

fn gen_report_req(list: Vec<ScanResult>) -> tokio_stream::Iter<IntoIter<ReportAppPackagesRequest>> {
    let infos: HashMap<String, AppPackageBinary> = list.into_iter().fold(
        HashMap::new(),
        |mut m: HashMap<String, AppPackageBinary>, r: ScanResult| {
            m.insert(
                r.name.to_owned(),
                AppPackageBinary {
                    name: r.name,
                    size_byte: r.size as i64,
                    public_url: "".to_owned(),
                },
            );
            m
        },
    );
    tokio_stream::iter(vec![ReportAppPackagesRequest {
        app_packages: infos,
    }])
}
