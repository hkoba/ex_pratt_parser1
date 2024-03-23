## これは何？

今朝読んだ @matklad 氏の [Pratt Parser の紹介記事](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)が猛烈に面白かったので、rust の復習兼ねてやってみただけ。

### 何を忘れてたか

- struct と impl に分けて書くことを忘れてる(C++脳だ)
- uchar じゃない、 char だった(C++脳だ)
- メソッドの引数宣言に `(self: &mut Self)` を忘れる

- 戻り型は `->` で書くんだった(typescript とごっちゃになってる)
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
