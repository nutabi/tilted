# Changelog for `tilted`

## Version 0.3.0

- Added interactive mode and printing of AST nodes to CLI [`7450cb16`]

- Added support for (trigonometric) functions [`23fbcdf8`]

- Added a new feature `serde` that supports serialisation and deserialisation with `serde` [`861ed36f`]

- Added support to convert `Node` to a human-readable tree [`21e72ecc`]

- Fixed an error that caused implicit multiplication to be parsed incorrectly [`1622f59d`]

- Fixed an error that caused floating-point number calculation and comparison to be invalid [`810367ee`]

- Improved clarity of both tokenisation and production rules

- Changed location of both unittests and integration tests to `tests`

[`7450cb16`]: https://github.com/SaltedPeanutButter/cal/commit/7450cb165dea47c73cccf3fd0910a0e4541d3055
[`23fbcdf8`]: https://github.com/SaltedPeanutButter/cal/commit/23fbcdf8bc38ad30725db7bbc723bcaef106afa6
[`861ed36f`]: https://github.com/SaltedPeanutButter/cal/commit/861ed36feb1b9c4f203777688406840c8c034d9a
[`21e72ecc`]: https://github.com/SaltedPeanutButter/cal/commit/21e72ecc653cfe7dab5a02c041cdd5cb9fc61540
[`1622f59d`]: https://github.com/SaltedPeanutButter/cal/commit/2f59d54b2ad2ec5ee2587febb0d97e38e170e6df
[`810367ee`]: https://github.com/SaltedPeanutButter/cal/commit/810367ee9682a943334f18c9b94eaca2740f9427

## Version 0.2.0

- Added a command-line interface [`1629b6d9`]

- Added support for a caret operator `^` and exponentiation [`9ecb628f`]

- Fixed an error-by-design that caused errors during lexical analysis to be ignored [`01dd2d6f`]

[`1629b6d9`]: https://github.com/SaltedPeanutButter/cal/commit/1629b6d9ad6cf17ec4e5924add407847bff929a4
[`9ecb628f`]: https://github.com/SaltedPeanutButter/cal/commit/9ecb628fc13d5e82c54a8c3cb27bf8dbd2aaa110
[`01dd2d6f`]: https://github.com/SaltedPeanutButter/cal/commit/01dd2d6f68d1c1a1794cf8576e45e0c10c4e9b54

## Version 0.1.1

- Added support for implicit multiplication [`b560ed8f`]

- Added support for multiple unary operators [`7f469f24`]

[`b560ed8f`]: https://github.com/SaltedPeanutButter/cal/commit/b560ed8f5b6202e5ffd145910cc117e826ad6c8f
[`7f469f24`]: https://github.com/SaltedPeanutButter/cal/commit/7f469f24a5aedb1f3b04dbbd0b95638b1e62057c
