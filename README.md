# percentile

Calculate percentiles on response time from logs.

## First have to build it!
```
vladimirbright@percentile2$ cargo build --release
```

## Get help.
```

vladimirbright@percentile2$ time ./target/release/percentile --help
Calculate response time percentiles on logs 0.0.1
Vladimir Prokhoda <vladimirbright@gmail.com>

USAGE:
    percentile [FLAGS] [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -p, --print      Print matched rows
    -V, --version    Prints version information

OPTIONS:
    -c, --column <column>          Column number to use [default: 8]
    -s, --separator <separator>    Column separator [default:  ]

ARGS:
    <input>    Sets the input file to use

real    0m0.011s
user    0m0.002s
sys     0m0.002s
```

## And we can run it.

```
vladimirbright@percentile2$ time ./target/release/percentile --separator=' ' --column=8 ../percentile/logs/access.log

Results:
    Total count:   20
    Min:           0.002
    Median:        0.003 (10)
    70 percentile: 0.016 (14)
    80 percentile: 0.017 (16)
    95 percentile: 0.026 (19)
    99 percentile: 0.026 (19)
    Max:           0.076 (20)

real    0m0.007s
user    0m0.002s
sys     0m0.002s
```
In parentheses index of percentile, a.k.a. number of requests under this percentile. Log format something like this (e.g. space separated and response time column on 8 index):

```
vladimirbright@percentile2$ tail ../percentile/logs/access.log
95.26.246.165 [25/Feb/2017:10:06:48 +0400] "GET /apple-touch-icon-precomposed.png HTTP/1.1" 404 259 0.002 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:48 +0400] "GET /apple-touch-icon.png HTTP/1.1" 404 259 0.002 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:48 +0400] "GET /favicon.ico HTTP/1.1" 404 259 0.002 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:48 +0400] "GET /apple-touch-icon-120x120-precomposed.png HTTP/1.1" 404 259 0.003 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:49 +0400] "GET /apple-touch-icon-120x120.png HTTP/1.1" 404 259 0.003 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:49 +0400] "GET /apple-touch-icon-precomposed.png HTTP/1.1" 404 259 0.004 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:49 +0400] "GET /apple-touch-icon.png HTTP/1.1" 404 259 0.003 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
95.26.246.165 [25/Feb/2017:10:06:49 +0400] "GET /favicon.ico HTTP/1.1" 404 259 0.003 "-" "MobileSafari/602.1 CFNetwork/808.3 Darwin/16.3.0"
213.180.206.198 [25/Feb/2017:10:59:28 +0400] "GET / HTTP/1.1" 200 1728 0.016 "-" "Mozilla/5.0 (compatible; YandexMetrika/2.0; +http://yandex.com/bots)"
66.249.64.136 [25/Feb/2017:11:56:33 +0400] "GET /robots.txt HTTP/1.1" 404 259 0.003 "-" "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
```
