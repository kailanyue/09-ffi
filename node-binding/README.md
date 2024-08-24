### 安装 node 和 npm
下载路径：[https://nodejs.org/dist](https://nodejs.org/dist/)，根据系统下载对应的版本

### 安装 yarn
```sh
# 全局安装
npm install -g yarn
```

### 克隆项目
从仓库 [napi-rs/package-template](https://github.com/napi-rs/package-template) 下载项目


### 编译 node-binding
```sh
cd node-binding
# 安装依赖
yarn
# 编译
yarn build
# 测试
yarn test
```

### 通过node 测试

```js
> const {Algo} = await import("./index.js");
undefined
> const algo1 = new Algo('sip');
undefined
> algo1.hash('hello world')
'8170069951894177743'
>
```
