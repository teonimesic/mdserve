# Images and Media Test

This document tests various image and media formats.

## Local Images

### PNG Image

![PNG Test Image](image.png)

### Relative Path Images

If you have images in subfolders, they should work too:

![Subfolder Image](images/test.jpg)

## External Images

### HTTPS Image

![External PNG](https://httpbin.org/image/png)

### SVG Image

![External SVG](https://httpbin.org/image/svg)

## Image with Link

[![Clickable Image](image.png)](https://example.com)

## Image Sizing (HTML)

<img src="image.png" alt="Small Image" width="50" height="50">

<img src="image.png" alt="Large Image" width="200">

## Multiple Images

![Image 1](image.png) ![Image 2](image.png) ![Image 3](image.png)

## Image in Blockquote

> This is a quote with an image:
>
> ![Quote Image](image.png)

## Image in List

- First item
- Second item with image:
  ![List Image](image.png)
- Third item

## Image in Table

| Description | Image |
|-------------|-------|
| Test Image | ![Table Image](image.png) |
| Another | ![Another](image.png) |
