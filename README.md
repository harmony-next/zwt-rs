# 图片占位服务使用说明

一个简单的图片占位服务，支持生成自定义尺寸、颜色和文本的占位图片。

## API 格式

1. 仅指定尺寸 

2. 指定尺寸和颜色
```
http://localhost:3000/{size}/{background-color}/{text-color}
```

3. 指定尺寸、颜色和文本
```
http://localhost:3000/{size}/{background-color}/{text-color}/text={text}
```

## 参数说明

- `size`: 图片尺寸
  - 单值: `500` (生成 500x500 的正方形)
  - 双值: `800x600` (生成 800x600 的矩形)

- `background-color`: 背景色，6位十六进制
  - 可选，默认为浅灰色 (f0f0f0)
  - 示例：ff0000 (红色)

- `text-color`: 文字颜色，6位十六进制
  - 可选，默认为黑色 (000000)
  - 示例：ffffff (白色)

- `text`: 显示的文本内容
  - 可选，默认显示图片尺寸
  - 支持中英文

## 使用示例

1. 生成默认颜色的正方形图片
```
http://localhost:3000/500
```

2. 生成指定尺寸的矩形图片
```
http://localhost:3000/800x600
```

3. 生成红底白字的图片
```
http://localhost:3000/800x600/ff0000/ffffff
```

4. 生成带自定义文本的图片
```
http://localhost:3000/800x600/ff0000/ffffff/text=Hello World
``` 
