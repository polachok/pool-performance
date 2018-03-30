The benchmark
-------------

Run this:

 ```ab -c 20 -t 30 http://127.0.0.1:1488/```

My results (mbp 2017, i5 2.3, 16Gb RAM):

```
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking 127.0.0.1 (be patient)
Finished 589 requests


Server Software:        
Server Hostname:        127.0.0.1
Server Port:            1488

Document Path:          /
Document Length:        0 bytes

Concurrency Level:      20
Time taken for tests:   30.130 seconds
Complete requests:      589
Failed requests:        0
Total transferred:      44250 bytes
HTML transferred:       0 bytes
Requests per second:    19.55 [#/sec] (mean)
Time per request:       1023.088 [ms] (mean)
Time per request:       51.154 [ms] (mean, across all concurrent requests)
Transfer rate:          1.43 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.2      0       1
Processing:   203 1001 571.3    812    2650
Waiting:      203 1001 571.3    812    2650
Total:        204 1001 571.3    813    2650

Percentage of the requests served within a certain time (ms)
  50%    813
  66%   1018
  75%   1420
  80%   1428
  90%   1835
  95%   2231
  98%   2435
  99%   2443
 100%   2650 (longest request)
 ```
