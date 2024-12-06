use futures::StreamExt;
use tokio::{self, sync::oneshot};

#[tokio::main]
async fn main() {
    let (term_tx, mut term_rx) = oneshot::channel();
    println!("This is action client");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_action_client", "").unwrap();
    let client = node
        .create_action_client::<r2r::example_interfaces::action::Fibonacci::Action>("/my_action")
        .unwrap();

    let spin_handler = tokio::task::spawn_blocking(move || {
        while term_rx.try_recv().is_err() {
            node.spin_once(std::time::Duration::from_millis(100));
        }
    });

    println!("Waiting for the action server ready");
    r2r::Node::is_available(&client).unwrap().await.unwrap();
    println!("Action server is ready");

    let my_goal = r2r::example_interfaces::action::Fibonacci::Goal { order: 5 };
    println!(r#"Send action goal "{:?}""#, my_goal);
    let (goal, mut result_fut, mut feedback) =
        client.send_goal_request(my_goal).unwrap().await.unwrap();

    loop {
        tokio::select! {
            msg = feedback.next() => {
                println!("feedback msg: {:?}, current status {:?}", msg, goal.get_status().unwrap());
            }
            result = &mut result_fut => {
                match result {
                    Ok((status, msg)) => {
                        println!("Received result {}, msg {:?}", status, msg);
                    }
                    Err(e) => {
                        println!("Action failed: {:?}", e);
                    }
                }
                break;
            }
        }
    }

    term_tx.send(()).unwrap();
    spin_handler.await.unwrap();
}
