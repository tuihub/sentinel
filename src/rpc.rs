use log::info;
use crate::Result;
use sentinel::greeter_client::GreeterClient;
use sentinel::ReportReq;
pub mod sentinel {
    tonic::include_proto!("sentinel");
}

pub async fn init(host: String, port: String, token: String) -> Result<()> {
    let mut client = GreeterClient::connect(format!("http://{}:{}", host, port)).await?;

    let request = tonic::Request::new(ReportReq {
        token
    });

    let response = client.report(request).await?;

    info!("Connectted to server, response {:?}", response);

    Ok(())
}