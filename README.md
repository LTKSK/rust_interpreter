このプロジェクトは、[Writing An Interpreter In Go](https://www.amazon.co.jp/Writing-Interpreter-English-Thorsten-Ball-ebook/dp/B01N2T1VD2) をRustで実装してみる学習用のプロジェクトです

作成に際して、以下の記事も参考しています
* [「Go言語でつくるインタプリタ」をRustで実装しました。](https://buildersbox.corp-sansan.com/entry/2020/06/29/110000)
* [[Rust] 『Go言語でつくるインタプリタ』Rustで読了](https://qiita.com/osanshouo/items/d1b18c90e06670d602fa)

# 実行方法

`docker image build -t interpreter .`  
`docker run -it --rm interpreter`
