use futures::StreamExt;
use r2r::{self, QosProfile};
use tokio;

#[tokio::main]
async fn main() {
    println!("This is service");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_sub", "").unwrap();
    let mut service = node
        .create_service::<r2r::std_srvs::srv::SetBool::Service>(
            "/my_service",
            QosProfile::default(),
        )
        .unwrap();

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    loop {
        match service.next().await {
            Some(req) => {
                println!(r#"Receive "{}""#, req.message.data);
                let resp = r2r::std_srvs::srv::SetBool::Response {
                    success: true,
                    message: "Success!".to_owned(),
                };
                println!(r#"Send back response "{:?}""#, resp);
                req.respond(resp).unwrap();
            }
            None => break,
        }
    }

    handle.await.unwrap();
}
