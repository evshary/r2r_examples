use futures::StreamExt;
use r2r::{self, QosProfile};
use tokio;

#[tokio::main]
async fn main() {
    println!("This is subscriber");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_sub", "").unwrap();
    let subscriber = node
        .subscribe::<r2r::std_msgs::msg::String>("my_topic", QosProfile::default())
        .unwrap();
    tokio::spawn(async move {
        subscriber
            .for_each(|msg| {
                println!(r#"Receive "{}""#, msg.data);
                futures::future::ready(())
            })
            .await
    });
    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
