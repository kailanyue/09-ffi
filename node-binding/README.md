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

### 通过 node 测试

```ts
> const {Algo} = await import("./index.js");
undefined
> const algo1 = new Algo('sip');
undefined
> algo1.hash('hello world')
'8170069951894177743'
>
```

```ts
> const {Matrix} = await import("./index.js");
undefined
> const m1 = new Matrix([[1, 2], [3, 4]]);
undefined
> m1.display()
'{1 2, 3 4}'
> const m2 = new Matrix([[5, 6], [7, 8]]);
undefined
> m2.display()
'{5 6, 7 8}'
> const m3 = m1.mul(m2);
undefined
> m3.display()
'{19 22, 43 50}'
>
```
