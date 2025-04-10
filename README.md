# Reproducer for load balancing

This repository demonstrates the load balancing behaviour of DigitalOcean App Platform. To test, run this repo with more than 1 instance size. We've used [hey](https://github.com/rakyll/hey) to test, by sending N requests.

Each instance has an unique and static instance id. That is attached to the response and log. After running hey (or any other tool/method to send requests), if you visit the url or curl it, you'll see the number of requests are quite similar with a very small delta.

```bash
hey -n 200 -c 4 -z 3m $host
```
