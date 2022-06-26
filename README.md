# ab-rs

This project is aimed to learn async programing.

## useage

```bash
# simple demo
cargo run https://static.zmis.me/web/api/test.json -n12 -c2

# get show http response body,add -debug params
cargo run https://static.zmis.me/web/api/test.json -n1 -c1 -debug

# form-data
cargo run http://127.0.0.1:8081/v1/anniversary/public-account/date -n1 -c1 -hdopen_id=10,id=2,set=3 -hmpost

# json, this payload will auto transform data to json, like {"open_id":10,"id":2,"set":3}
cargo run http://127.0.0.1:8081/v1/anniversary/public-account/date -n1 -c1 -hcjson -hdopen_id=10,id=2,set="3" -hmpost

```

## command help

```bash
`-n` 请求数
`-c` 并发数
`-hd` http的post的body,用逗号分隔,json与非json同理：a=121312312,b=2，会自动解析成json或者form-data
`-hm` http的请求方法，[get]or[post]
`-hc` http的contentType, [json]or 非json,默认为form-data
`-debug` http的debug模式，是否显示返回的http body
```

## run to build

```bash
cargo build --release
```
