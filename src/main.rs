use sentinel::greeter_client::GreeterClient;
use sentinel::ReportReq;

pub mod sentinel {
    tonic::include_proto!("sentinel");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ReportReq {
        token: "".into()
    });

    let response = client.report(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}