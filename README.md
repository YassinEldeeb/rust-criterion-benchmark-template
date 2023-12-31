> [!IMPORTANT]
> #### Remember to give github actions write access through your repostiory settings, it only has read permissions by default.

# [Something] Benchmarks 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise, though, we can make a bold assumption that noise that will affect one will affect others. Since the benchmark only lasts a few seconds each.
        

| Benchmark | Time (in µs) |
|-----------|------|


![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Something1              time:   [309.22 ps 309.80 ps 310.68 ps]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high severe

Something2              time:   [309.07 ps 309.21 ps 309.38 ps]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe

Something3              time:   [309.46 ps 312.19 ps 315.41 ps]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe


```



</details>
