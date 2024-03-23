## これは何？

This project is a personal practice implementation inspired by [minipratt](https://github.com/matklad/minipratt) and [the accompanying blog post on Pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) by Aleksey Kladov. It's not entirely original but serves as an educational exercise in understanding and applying concepts from the article.

### 何を忘れてたか

- struct と impl に分けて書くことを忘れてる(C++脳だ)
- uchar じゃない、 char だった(C++脳だ)
- メソッドの引数宣言に `(self: &mut Self)` を忘れる

- 戻り型は `->` で書くんだった(typescript の `:` とごっちゃになってる)
- `.chars()` 多分初めて使った
  ```
  std::str::Chars<'_>
  std::iter::Peekable
  ```
- lifetime を書く場所忘れがち
  ```
  & 'a str
  ```
- library なら pub 立てようね
- match c {} とか、中の `x => {}, ...` とかも忘れてる
- 範囲オペレータ、末尾の `=` を忘れがち(Perl脳だ)
  ```
  '0' ..= '9'
  ```
- Option::map_or を忘れてる
- enum の `derive(Debug, Clone, Copy, PartialEq, Eq)` 、これは覚えたほうが良さそう

- mod の書き方も忘れてる。
- test 周り
  - `use super::*` 忘れてる
- Display 周り
  - `write!(f, "fmt", val)?;` 忘れてる
  - `.to_string()` もだ
- `println!(fmt, ...)` も忘れてやんの…
- `cargo test` から println したいときは↓こうだと
  ```sh
  cargo test -- --nocapture
  ```
