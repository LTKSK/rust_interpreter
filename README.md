このプロジェクトは、[Writing An Interpreter In Go](https://www.amazon.co.jp/Writing-Interpreter-English-Thorsten-Ball-ebook/dp/B01N2T1VD2) をRustで実装してみる学習用のプロジェクトです

作成に際して、以下の記事も参考しています
* [「Go言語でつくるインタプリタ」をRustで実装しました。](https://buildersbox.corp-sansan.com/entry/2020/06/29/110000)
* [[Rust] 『Go言語でつくるインタプリタ』Rustで読了](https://qiita.com/osanshouo/items/d1b18c90e06670d602fa)

# 実行方法

`docker image build -t interpreter .`  
`docker run -it --rm interpreter`

# 対応している文法

- 変数定義
  - `let a = 10`
- 四則演算
  - `10 + 3 + 2 * (1 + 2) #=> 19`
- 文字列の結合
  - `"aaa" + "bbb" #=> aaabbb`
- 配列の定義と参照
  - `let array = [1,2,3]`
  - `array[0] #=> 1`
- 連想配列の定義と参照
  - `let dict = {"a": 1, "b": "value"}`
  - `dict["a"] #=> 1`
  - `dict["b"] #=> value`
- 関数の定義と呼び出し
  - `let add = fn(a,b) { a + b }`
  - `add(1,2) #=> 3`
- for文
  - `let sum = 0; for val in [1,2,3] { sum = sum + val }; sum #=> 6`
- exit
  - `exit`
