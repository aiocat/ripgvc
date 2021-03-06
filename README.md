<!--
 Copyright (c) 2022 aiocat

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# RipGVC

Profile visit counter for GitHub.
![IRL](https://i.imgur.com/UtfV4yV.png)

## Introduction

RipGVC is a service that allows you to embed a `SVG` to your `README.md` file and count the visitors. This project made for being an alternative to [GPVC](https://github.com/antonkomarev/github-profile-views-counter).

This service aims to be fast as possible.

## Example

![Example](https://ripgvc.herokuapp.com/?username=your-github-username&color=e64f00&round)

You can use it like this:

```md
![](https://ripgvc.herokuapp.com/?username=aiocat)
```

Also you can customize your card color by passing `color` parameter:
_Any six-digit hexadecimal color is valid._
```diff
- ![](https://ripgvc.herokuapp.com/?username=aiocat)
+ ![](https://ripgvc.herokuapp.com/?username=aiocat&color=e64f00)
```

And, you can make the left side round if you want to:
```diff
- ![](https://ripgvc.herokuapp.com/?username=aiocat&color=e64f00)
+ ![](https://ripgvc.herokuapp.com/?username=aiocat&color=e64f00&round)
```

## Technologies

- Rust with `axum` Back-End
- MongoDB

## Contributing

All pull-requests and issues are welcome. Just make sure you got a brain.

If you got an error, Please open an issue at [here](https://github.com/aiocat/ripgvc/issues).

## Building

### Pre-Requests

- Rust compiler and Cargo must be installed to your computer

### Progress

- Clone the repo (`git clone git@github.com:aiocat/ripgvc.git`)
- Move into folder (`cd ripgvc`)
- Run cargo build (`cargo build --release`)

## License

RipGVC is distributed under the MIT license. for more information:

- https://raw.githubusercontent.com/aiocat/ripgvc/main/LICENSE
