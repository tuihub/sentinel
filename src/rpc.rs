use std::{collections::HashMap, vec::IntoIter};

use crate::{scanner::ScanResult, Result};
use log::debug;

use tonic::transport::Channel;
use tuihub_protos::librarian::{
    sephirah::v1::{
        librarian_sephirah_service_client::LibrarianSephirahServiceClient, ReportAppPackageRequest,
    },
    v1::AppPackageBinary,
};

pub struct Client {
    rpc: LibrarianSephirahServiceClient<Channel>,
    token: String,
}

pub async fn init(host: String, port: String, token: String) -> Result<Client> {
    let client =
        LibrarianSephirahServiceClient::connect(format!("http://{}:{}", host, port)).await?;

    Ok(Client { rpc: client, token })
}

impl Client {
    pub async fn report(mut self, list: Vec<ScanResult>) -> Result<()> {
        let mut request = tonic::Request::new(gen_report_req(list));
        request
            .metadata_mut()
            .insert("authorization", format!("token {}", self.token).parse()?);
        debug!("ReportReq: {:?}", request);
        let response = self.rpc.report_app_package(request).await?;
        debug!("ReportResp: {:?}", response);
        Ok(())
    }
}

fn gen_report_req(list: Vec<ScanResult>) -> tokio_stream::Iter<IntoIter<ReportAppPackageRequest>> {
    let infos: HashMap<String, AppPackageBinary> = list.into_iter().fold(
        HashMap::new(),
        |mut m: HashMap<String, AppPackageBinary>, r: ScanResult| {
            m.insert(
                r.name.to_owned(),
                AppPackageBinary {
                    name: r.name,
                    size: r.size.to_string(),
                    public_url: "".to_owned(),
                },
            );
            m
        },
    );
    tokio_stream::iter(vec![ReportAppPackageRequest {
        app_package_list: infos,
    }])
}
