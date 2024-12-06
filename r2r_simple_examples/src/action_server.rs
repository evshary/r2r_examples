use futures::StreamExt;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("This is action server");

    let ctx = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(ctx, "ros_action_server", "").unwrap();
    let mut action_server = node
        .create_action_server::<r2r::example_interfaces::action::Fibonacci::Action>("/my_action")
        .unwrap();

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    loop {
        match action_server.next().await {
            Some(req) => {
                println!(
                    r#"Receive goal request with order {}, goal id: {}"#,
                    req.goal.order, req.uuid
                );

                let (mut recv_goal, mut cancel) = req.accept().unwrap();

                let recv_goal_clone = recv_goal.clone();
                let fut = async move {
                    let mut feedback_data = r2r::example_interfaces::action::Fibonacci::Feedback {
                        sequence: vec![0, 1],
                    };
                    recv_goal_clone
                        .publish_feedback(feedback_data.clone())
                        .unwrap();

                    let order = recv_goal_clone.goal.order as usize;
                    for i in 1..order {
                        feedback_data
                            .sequence
                            .push(feedback_data.sequence[i] + feedback_data.sequence[i - 1]);
                        println!("Sending feedback: {:?}", feedback_data);
                        recv_goal_clone
                            .publish_feedback(feedback_data.clone())
                            .unwrap();
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }

                    r2r::example_interfaces::action::Fibonacci::Result {
                        sequence: feedback_data.sequence,
                    }
                };

                tokio::select! {
                    final_result = fut => {
                        println!("Complete! Final result is sent.");
                        recv_goal.succeed(final_result).unwrap();

                    }
                    _ = cancel.next() => {
                        println!("Cancelled by users.");
                    }
                };
            }
            None => break,
        }
    }

    handle.await.unwrap();
}
