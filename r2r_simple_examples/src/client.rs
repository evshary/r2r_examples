use r2r::{self, QosProfile};
use tokio::{self, sync::oneshot};

#[tokio::main]
async fn main() {
    let (term_tx, mut term_rx) = oneshot::channel();
    let req_data = true;
    println!("This is client");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_client", "").unwrap();
    let client = node
        .create_client::<r2r::std_srvs::srv::SetBool::Service>("/my_service", QosProfile::default())
        .unwrap();

    let handler = tokio::task::spawn_blocking(move || {
        while term_rx.try_recv().is_err() {
            node.spin_once(std::time::Duration::from_millis(100));
        }
    });

    println!("Waiting for the service ready");
    r2r::Node::is_available(&client).unwrap().await.unwrap();
    println!("Service is ready");

    let my_req = r2r::std_srvs::srv::SetBool::Request { data: req_data };
    println!(r#"Send Request "{}""#, req_data);
    let resp = client.request(&my_req).unwrap().await.unwrap();
    println!(r#"Receive Response "{:?}""#, resp);

    term_tx.send(()).unwrap();
    handler.await.unwrap();
}
