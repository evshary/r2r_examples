# r2r-examples

The repository includes some simple examples to use r2r.

* Install pre-commit

```shell
pre-commit install --install-hooks
```

* Build

```shell
cargo build
```

* Run examples

```shell
# Publisher
ros2 run r2r_simple_examples pub
# Subscriber
ros2 run r2r_simple_examples sub
# Service
ros2 run r2r_simple_examples service
# Client
ros2 run r2r_simple_examples client
# Action Service
ros2 run r2r_simple_examples action_server
# Action Client
ros2 run r2r_simple_examples action_client
```
