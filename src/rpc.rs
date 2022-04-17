use log::{info, debug};
use tonic::transport::Channel;
use crate::Result;
use sentinel::sentinel_service_client::SentinelServiceClient;
use sentinel::{ReportReq, ReportInfo};

pub mod sentinel {
    tonic::include_proto!("sentinel");
}

pub struct Client{
    rpc: SentinelServiceClient<Channel>,
    token: String,
}

pub async fn init(host: String, port: String, token: String) -> Result<Client> {
    let mut client = SentinelServiceClient::connect(format!("http://{}:{}", host, port)).await?;

    let request = tonic::Request::new(gen_report_req(token.clone(), None));

    let response = client.report(request).await?;

    info!("Connectted to server, response {:?}", response);

    Ok(Client{
        rpc: client,
        token
    })
}

impl Client {
    pub async fn report(mut self, list: Vec<String>) -> Result<()> {
        let request = tonic::Request::new(gen_report_req(self.token, Some(list)));
        debug!("ReportReq: {:?}", request);
        let response = self.rpc.report(request).await?;
        debug!("ReportResp: {:?}", response);
        Ok(())
    }
}

fn gen_report_req(token: String, list: Option<Vec<String>>) -> ReportReq {
    let list = list.unwrap_or_default();
    let infos: Vec<ReportInfo> = list.into_iter()
        .map(|file_name| ReportInfo{file_name, file_size: None}).collect();
    ReportReq { 
        token, 
        infos,
    }
}