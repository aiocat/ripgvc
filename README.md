<!--
 Copyright (c) 2022 aiocat
 
 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# RipGVC
Profile visit counter for GitHub.

## Introduction
RipGVC is a service that allows you to embed a `SVG` to your `README.md` file and count the visitors. This project made for being an alternative to [GPVC](https://github.com/antonkomarev/github-profile-views-counter).

This service aims to be fast as possible.

## Example
![](https://ripgvc.herokuapp.com/?username=asd&color=ff6c1f)

You can use it like this:
```md
![](https://ripgvc.herokuapp.com/?username=aiocat)
```
Also you can customize your card color by passing `color` parameter:
```md
![](https://ripgvc.herokuapp.com/?username=aiocat&color=ff6c1f)
```
Any six-digit hexadecimal color is valid.