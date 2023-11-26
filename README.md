# HTTP Server Performance Comparison

This document provides a performance comparison between `Actix-Web`, `Hyper`, `async_uws` and `Rocket` using K6 benchmarks.

## Performance Summary Table

| Metric                    | Rocket       | Actix-Web    | Hyper        | uWS          |
|---------------------------|--------------|--------------|--------------|--------------|
| Total Requests            | 1,366,528    | 1,110,920    | 942,697      | 17,055       |
| Success Rate              | 99.99%       | 99.99%       | 99.99%       | 95.54%       |
| Throughput (reqs/sec)     | ~45,551      | ~37,027      | ~31,418      | ~481         |
| Avg Request Duration (ms) | 3.67         | 4.56         | 5.54         | 13.71        |
| Errors                    | 68           | 61           | 55           | 760          |
| Peak Response Time (ms)   | 107.56       | 105.96       | 62.37        | 726.41       |
| VUs                       | 200          | 200          | 200          | 13-200       |
| VUs Max                   | 200          | 200          | 200          | 200          |


<details>
  <summary>K6 Raw Summary for Rocket</summary>


✗ response code was 200
↳ 99% — ✓ 1,366,528 / ✗ 68
✗ response body is 'Hello World'
↳ 99% — ✓ 1,366,528 / ✗ 68

checks.........................: 99.99% ✓ 2,733,056 ✗ 136
data_received..................: 336 MB 11 MB/s
data_sent......................: 109 MB 3.6 MB/s
http_req_blocked...............: avg=3.06µs min=0s med=1µs max=50.01ms p(90)=1µs p(95)=2µs
http_req_connecting............: avg=1.15µs min=0s med=0s max=11.03ms p(90)=0s p(95)=0s
http_req_duration..............: avg=3.67ms min=31µs med=2.44ms max=107.56ms p(90)=8.04ms p(95)=10.94ms
http_req_failed................: 0.00% ✓ 68 ✗ 1,366,528
http_req_receiving.............: avg=25.17µs min=0s med=7µs max=92.12ms p(90)=13µs p(95)=27µs
http_req_sending...............: avg=10.2µs min=1µs med=2µs max=65.25ms p(90)=5µs p(95)=7µs
http_req_tls_handshaking.......: avg=0s min=0s med=0s max=0s p(90)=0s p(95)=0s
http_req_waiting...............: avg=3.63ms min=25µs med=2.42ms max=107.5ms p(90)=7.99ms p(95)=10.86ms
http_reqs......................: 1,366,596 45,551.022661/s
iteration_duration.............: avg=4.28ms min=45.2µs med=2.86ms max=109.38ms p(90)=9.18ms p(95)=12.73ms
iterations.....................: 1,366,596 45,551.022661/s
vus............................: 200 min=200 max=200
vus_max........................: 200 min=200 max=200

</details>


<details>
  <summary>K6 Raw Summary for Actix-Web</summary>

✗ response code was 200
↳ 99% — ✓ 1110859 / ✗ 61
✗ response body is 'Hello World'
↳ 99% — ✓ 1110859 / ✗ 61

checks.........................: 99.99% ✓ 2221718 ✗ 122
data_received..................: 97 MB 3.2 MB/s
data_sent......................: 89 MB 3.0 MB/s
http_req_blocked...............: avg=3.34µs min=0s med=1µs max=51.94ms p(90)=1µs p(95)=2µs
http_req_connecting............: avg=1.3µs min=0s med=0s max=7.32ms p(90)=0s p(95)=0s
http_req_duration..............: avg=4.56ms min=16µs med=2.48ms max=105.96ms p(90)=11.89ms p(95)=15.57ms
http_req_failed................: 0.00% ✓ 61 ✗ 1110859
http_req_receiving.............: avg=25.35µs min=0s med=6µs max=59.24ms p(90)=14µs p(95)=25µs
http_req_sending...............: avg=9.29µs min=1µs med=2µs max=46.33ms p(90)=4µs p(95)=7µs
http_req_tls_handshaking.......: avg=0s min=0s med=0s max=0s p(90)=0s p(95)=0s
http_req_waiting...............: avg=4.53ms min=12µs med=2.45ms max=105.95ms p(90)=11.82ms p(95)=15.49ms
http_reqs......................: 1110920 37027.172536/s
iteration_duration.............: avg=5.27ms min=30.95µs med=3.05ms max=105.98ms p(90)=13.17ms p(95)=17.2ms
iterations.....................: 1110920 37027.172536/s
vus............................: 200 min=200 max=200
vus_max........................: 200 min=200 max=200

</details>

<details>
  <summary>K6 Raw Summary for Hyper</summary>

✗ response code was 200
↳ 99% — ✓ 942,642 / ✗ 55
✗ response body is 'Hello World'
↳ 99% — ✓ 942,642 / ✗ 55

checks.........................: 99.99% ✓ 1,885,284 ✗ 110
data_received..................: 82 MB 2.7 MB/s
data_sent......................: 75 MB 2.5 MB/s
http_req_blocked...............: avg=2.73µs min=0s med=1µs max=47.44ms p(90)=1µs p(95)=2µs
http_req_connecting............: avg=1.38µs min=0s med=0s max=7.74ms p(90)=0s p(95)=0s
http_req_duration..............: avg=5.54ms min=17µs med=3.61ms max=62.37ms p(90)=13.37ms p(95)=16.87ms
http_req_failed................: 0.00% ✓ 55 ✗ 942,642
http_req_receiving.............: avg=31.5µs min=0s med=6µs max=62.34ms p(90)=16µs p(95)=28µs
http_req_sending...............: avg=9.02µs min=1µs med=2µs max=42.8ms p(90)=5µs p(95)=8µs
http_req_tls_handshaking.......: avg=0s min=0s med=0s max=0s p(90)=0s p(95)=0s
http_req_waiting...............: avg=5.5ms min=11µs med=3.58ms max=62.12ms p(90)=13.29ms p(95)=16.77ms
http_reqs......................: 942,697 31,418.492283/s
iteration_duration.............: avg=6.19ms min=30.08µs med=4.16ms max=82.2ms p(90)=14.43ms p(95)=18.32ms
iterations.....................: 942,697 31,418.492283/s
vus............................: 200 min=200 max=200
vus_max........................: 200 min=200 max=200


</details>

<details>
  <summary>K6 Raw Summary for async_uws</summary>

✗ response code was 200
↳ 95% — ✓ 16,295 / ✗ 760
✗ response body is 'Hello World'
↳ 95% — ✓ 16,295 / ✗ 760

checks.........................: 95.54% ✓ 32,590 ✗ 1,520
data_received..................: 2.0 MB 57 kB/s
data_sent......................: 1.4 MB 38 kB/s
http_req_blocked...............: avg=129.49ms min=0s med=6.85ms max=19.5s p(90)=8.95ms p(95)=10.07ms
http_req_connecting............: avg=129.46ms min=0s med=6.82ms max=19.5s p(90)=8.9ms p(95)=10.02ms
http_req_duration..............: avg=13.71ms min=0s med=7.46ms max=726.41ms p(90)=9.98ms p(95)=12.36ms
http_req_failed................: 4.45% ✓ 760 ✗ 16,295
http_req_receiving.............: avg=166.76µs min=0s med=43µs max=20.36ms p(90)=145µs p(95)=309.29µs
http_req_sending...............: avg=124.01µs min=0s med=8µs max=19.92ms p(90)=61µs p(95)=340.29µs
http_req_tls_handshaking.......: avg=0s min=0s med=0s max=0s p(90)=0s p(95)=0s
http_req_waiting...............: avg=13.42ms min=0s med=7.31ms max=724.6ms p(90)=9.59ms p(95)=11.18ms
http_reqs......................: 17,055 480.658992/s
iteration_duration.............: avg=406.08ms min=327.54µs med=14.64ms max=25.92s p(90)=17.93ms p(95)=24.93ms
iterations.....................: 17,055 480.658992/s
vus............................: 13 min=13 max=200
vus_max........................: 200 min=200 max=200

</details>
