use r2r::{self, QosProfile};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let payload = "Hello World";
    println!("This is publisher");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_pub", "").unwrap();
    let publisher = node
        .create_publisher("my_topic", QosProfile::default())
        .unwrap();
    let msg = r2r::std_msgs::msg::String {
        data: payload.into(),
    };
    loop {
        println!(r#"Publish "{}""#, payload);
        publisher.publish(&msg).unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
