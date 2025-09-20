<p align="center">
	<a href="https://naccl.top/" target="_blank">
		<img src="./blog-view/public/img/qr.png" alt="ZeroBlog logo" style="width: 200px; height: 200px">
	</a>
</p>
<p align="center">
	<img src="https://img.shields.io/badge/RUST-1.85-orange">
	<img src="https://img.shields.io/badge/Vue-2.6.11-brightgreen">
	<img src="https://img.shields.io/badge/license-MIT-blue">
</p>

## 简介

dream_blog 是一个基于 Rust (actix-web) + Vue 的前后端分离博客系统，自用博客，长期维护中。

本项目是从原 Java 版本的 NBlog 移植而来的 Rust 实现版本，旨在学习和实践 Rust 开发 Web 应用。

**项目状态**：开发阶段

## 后端技术栈

1. 核心框架：[actix-web](https://github.com/actix/actix-web) - 高性能的 Rust Web 框架
2. ORM 框架：[sea-orm](https://github.com/SeaQL/sea-orm) - 异步 ORM 框架
3. 数据库：MySQL
4. NoSQL 缓存：[Redis](https://github.com/redis/redis)
5. 认证与授权：[actix-jwt-session](https://github.com/orhanbalci/actix-jwt-session)
6. Markdown 解析：[comrak](https://github.com/kivikakk/comrak) - CommonMark 规范解析器
7. 日志系统：[log4rs](https://github.com/estk/log4rs)
8. 序列化/反序列化：[serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json)
9. UserAgent 解析：[user-agent-parser](https://github.com/magiclen/user-agent-parser)

## 前端技术栈

核心框架：Vue2.x、Vue Router、Vuex

Vue 项目基于 @vue/cli4.x 构建

JS 依赖及参考的 css：[axios](https://github.com/axios/axios)、[moment](https://github.com/moment/moment)、[nprogress](https://github.com/rstacruz/nprogress)、[v-viewer](https://github.com/fengyuanchen/viewerjs)、[prismjs](https://github.com/PrismJS/prism)、[APlayer](https://github.com/DIYgod/APlayer)、[MetingJS](https://github.com/metowolf/MetingJS)、[lodash](https://github.com/lodash/lodash)、[mavonEditor](https://github.com/hinesboy/mavonEditor)、[echarts](https://github.com/apache/echarts)、[tocbot](https://github.com/tscanlin/tocbot)、[iCSS](https://github.com/chokcoco/iCSS)

### 后台 UI

后台基于 [vue-admin-template](https://github.com/PanJiaChen/vue-admin-template) 二次修改后的 [my-vue-admin-template](https://github.com/Naccl/my-vue-admin-template) 模板进行开发

UI 框架为 [Element UI](https://github.com/ElemeFE/element)

### 前台 UI

[Semantic UI](https://semantic-ui.com/)：主要使用，页面布局样式，适合前台界面的开发，语义化的 CSS

[Element UI](https://github.com/ElemeFE/element)：部分使用，一些小组件，弥补了 Semantic UI 的不足

文章排版：基于 [typo.css](https://github.com/sofish/typo.css) 修改

## 开发环境

1. `git clone` 项目
2. 创建 MySQL 数据库 `zero_blog`，并执行 `sql` 文件初始化表数据
3. 安装 Redis 并启动
4. 修改配置文件 `config.yaml` 确认 MySQL 和 Redis 数据库用户密码
5. `cargo run` 启动后端服务
6. 分别在 `blog-cms` 和 `blog-view` 目录下执行 `npm install` 安装依赖
7. 分别在 `blog-cms` 和 `blog-view` 目录下执行 `npm run serve` 启动前后台页面

## 项目进度

### 已完成功能
- 日志系统
- 数据库连接与模型 (Sea-orm)
- JWT 认证授权
- 前台页面
  - 首页
  - 关于
  - 分类
  - 标签
  - 友链
  - 归档
  - 动态
  - 评论
- Redis 缓存（进行中）

### 开发中功能
- 后台页面
  - 首页
  - 分类
  - 标签
  - 友链
  - 文章
  - 动态

## 隐藏功能

- 在前台访问 `/login` 路径登录后，可以以博主身份（带有博主标识）回复评论，且不需要填写昵称和邮箱即可提交
- 在 Markdown 中加入 `<meting-js server="netease" type="song" id="歌曲id" theme="#25CCF7"></meting-js>` （注意以正文形式添加，而不是代码片段）可以在文章中添加 [APlayer](https://github.com/DIYgod/APlayer) 音乐播放器，`netease` 为网易云音乐，其它配置及具体用法参考 [MetingJS](https://github.com/metowolf/MetingJS)
- 提供了两种隐藏文字效果：在 Markdown 中使用 `@@` 包住文字，文字会被渲染成"黑幕"效果，鼠标悬浮在上面时才会显示；使用 `%%` 包住文字，文字会被"蓝色覆盖层"遮盖，只有鼠标选中状态才会反色显示。例如：`@@隐藏文字@@`，`%%隐藏文字%%`

## LICENSE

[MIT](./LICENSE)

## Thanks

感谢 [Nblog](https://github.com/Naccl/NBlog) 项目中的前端页面

感谢 [JetBrains](https://www.jetbrains.com/?from=ZeroBlog) 提供的 Open Source License

感谢上面提到的每个开源项目