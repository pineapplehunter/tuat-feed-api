# 農工大WEB掲示板API

[![Check and test](https://github.com/pineapplehunter/tuat-feed-api/actions/workflows/check_test.yml/badge.svg)](https://github.com/pineapplehunter/tuat-feed-api/actions/workflows/check_test.yml)
[![codecov](https://codecov.io/gh/pineapplehunter/tuat-feed-api/branch/main/graph/badge.svg?token=2R060ZQDDE)](https://codecov.io/gh/pineapplehunter/tuat-feed-api)

## バージョン2
クエリによって選択できるようにしました。
https://api.ihavenojob.work/tuat/v2/

### 学部(gakubu)
* 工学部
  * `Technology`
* 農学部
  * `Agriculture`

### 情報の種類(category)
* 学生生活情報
  * `Campus`
* 教務情報
  * `Academic`
* すべて
  * `All`

### クエリのサンプル

工学部の教務情報
https://api.ihavenojob.work/tuat/v2/?gakubu=Technology&category=Academic

農学部の学生生活情報
https://api.ihavenojob.work/tuat/v2/?gakubu=Agriculture&category=Campus

工学部のすべて
https://api.ihavenojob.work/tuat/v2/?gakubu=Technology&category=All

### 情報の種類

## バージョン1（古い方）
### 工学部

教務情報と学生生活情報 https://api.ihavenojob.work/tuat/T

教務情報のみ: https://api.ihavenojob.work/tuat/T/academic

学生生活情報のみ: https://api.ihavenojob.work/tuat/T/campus

### 農学部

教務情報と学生生活情報 https://api.ihavenojob.work/tuat/A

教務情報のみ: https://api.ihavenojob.work/tuat/A/academic

学生生活情報のみ: https://api.ihavenojob.work/tuat/A/campus
