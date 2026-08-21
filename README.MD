<p align="center">
	<a href="https://lurendie.com/" target="_blank">
		<img src="./blog-view/public/img/qr.png" alt="Dream Blog logo" style="width: 200px; height: 200px">
	</a>
</p>
<p align="center">
	<img src="https://img.shields.io/badge/RUST-1.85-orange">
	<img src="https://img.shields.io/badge/Vue-3.5-brightgreen">
	<img src="https://img.shields.io/badge/license-MIT-blue">
</p>

## 简介

Dream Blog是一个基于 Rust (actix-web) + Vue 的前后端分离博客系统，自用博客，长期维护中。

本项目是从原 Java 版本的 [Nblog](https://github.com/Naccl/NBlog) 移植而来的 Rust 实现版本，旨在学习和实践 Rust 开发 Web 应用。

**项目状态**：开发阶段

## 后端技术栈

1. 核心框架：[actix-web](https://github.com/actix/actix-web) - 高性能的 Rust Web 框架
2. ORM 框架：[sea-orm](https://github.com/SeaQL/sea-orm) - 异步 ORM 框架
3. 数据库：MySQL
4. NoSQL 缓存：[Redis](https://github.com/redis/redis)
5. 认证与授权：[actix-jwt-session](https://github.com/orhanbalci/actix-jwt-session)
6. Markdown 解析：[comrak](https://github.com/kivikakk/comrak) - CommonMark 规范解析器
7. 日志系统：[tracing](https://github.com/tokio-rs/tracing) + [tracing-subscriber](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber)（stdout + 滚动文件，可用 `RUST_LOG` 环境变量控制级别与模块过滤）
8. 序列化/反序列化：[serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json)
9. UserAgent 解析：[user-agent-parser](https://github.com/magiclen/user-agent-parser)

## 前端技术栈

核心框架：Vue 3.x、Vue Router、Vuex、Vite

前端项目基于 Vite 构建（blog-cms、blog-view 分别为 `vite.config.js` / `vite.config.mjs`）

JS 依赖及参考的 css：[axios](https://github.com/axios/axios)、[dayjs](https://github.com/iamkun/dayjs)、[nprogress](https://github.com/rstacruz/nprogress)、[v-viewer](https://github.com/fengyuanchen/viewerjs)、[prismjs](https://github.com/PrismJS/prism)、[APlayer](https://github.com/DIYgod/APlayer)、[MetingJS](https://github.com/metowolf/MetingJS)、[lodash](https://github.com/lodash/lodash)、[md-editor-v3](https://github.com/imzbf/md-editor-v3)、[echarts](https://github.com/apache/echarts)、[tocbot](https://github.com/tscanlin/tocbot)、[iCSS](https://github.com/chokcoco/iCSS)

### Markdown 编辑器

后台写文章、写动态、关于页和友链页的 Markdown 编辑器使用 [md-editor-v3](https://github.com/imzbf/md-editor-v3)，对应依赖位于 `blog-cms/package.json`，并在 `blog-cms/src/main.js` 中引入编辑器样式。

当前未使用 `mavonEditor`。`mavonEditor` 更偏 Vue 2 生态，老项目迁移成本低；本项目后台基于 Vue 3 + Vite，`md-editor-v3` 对 Vue 3 支持更直接，组件按需引入更清晰，也更适合当前的前端构建方式。

编辑器能力主要用于 Markdown 写作、预览、图片上传和内容回显。后端会统一处理 Markdown 渲染、安全 HTML 过滤、文章描述兜底提取和 SEO 元信息生成，避免编辑器与前台展示逻辑强耦合。

### 后台 UI

后台基于 [vue-admin-template](https://github.com/PanJiaChen/vue-admin-template) 二次修改后的 [my-vue-admin-template](https://github.com/Naccl/my-vue-admin-template) 模板进行开发

UI 框架为 [Element Plus](https://github.com/element-plus/element-plus)

### 前台 UI

[Semantic UI](https://semantic-ui.com/)：主要使用，页面布局样式，适合前台界面的开发，语义化的 CSS

[Element Plus](https://github.com/element-plus/element-plus)：部分使用，一些小组件，弥补了 Semantic UI 的不足

文章排版：基于 [typo.css](https://github.com/sofish/typo.css) 修改

## 开发环境

1. `git clone` 项目
2. 创建 MySQL 数据库 `blog_api`，并执行 `sql` 文件初始化表数据
3. 安装 Redis 并启动
4. 修改后端配置文件 `blog-api/config/app_config.yaml`（推荐：复制 `blog-api/config/app_config.local.yaml.example` 为 `blog-api/config/app_config.local.yaml` 填入本地私密信息）确认 MySQL、Redis 等配置
5. （可选）日志级别过滤：设置 `RUST_LOG` 环境变量（如 `RUST_LOG=info,blog_api=debug`），日志默认输出到 stdout 与 `logs/blog-api.log`（按天滚动）
6. `cargo run` 启动后端服务
7. 分别在 `blog-cms` 和 `blog-view` 目录下执行 `npm install` 安装依赖
8. 分别在 `blog-cms` 和 `blog-view` 目录下执行 `npm run serve` 启动前后台页面
9. （可选）后台“定时任务”中开启 `local / stats.aggregate_visit`，用于定期聚合 PV/UV 与城市访客统计

## 部署说明

前台 `blog-view` 构建后会输出带 hash 的静态资源。Nginx 可参考 `blog-view/deploy/nginx-cache.conf` 配置缓存策略：`/assets` 使用长期 immutable 缓存，`index.html` 不缓存，普通图片、字体和脚本资源使用较长缓存。

部署时需要确保后端接口、前台站点地址、GitHub/又拍云/腾讯云图床配置、Redis、MySQL 等配置项已按实际环境填写。私密配置建议写入 `blog-api/config/app_config.local.yaml`，不要提交真实密钥。

## 项目进度

标识说明：`[x]` 已完成，`[~]` 持续优化中，`[ ]` 未开发或待补齐。

### 已完成功能

- [x] 后端基础能力
  - [x] Actix Web HTTP 服务与统一响应结构
  - [x] SeaORM 数据库连接、实体模型与业务服务分层
  - [x] MySQL 数据持久化
  - [x] Redis 缓存接入
  - [x] JWT / httpOnly Cookie 登录认证
  - [x] 博主身份评论、密码保护文章访问控制
  - [x] Markdown 渲染、安全 HTML 过滤与文章描述兜底提取
  - [x] 统一错误处理、访问日志、操作日志、异常日志、登录日志
  - [x] 定时任务与访问统计聚合
  - [x] Sitemap 与 SEO 基础元信息
- [x] 前台页面 `blog-view`
  - [x] 首页、文章详情、分类、标签、归档、动态、友链、关于页
  - [x] 评论列表、评论提交、表情渲染、评论回复
  - [x] 文章目录、代码高亮、图片懒加载、图片预览
  - [x] 私密文章密码弹窗
  - [x] APlayer / MetingJS 音乐播放器按需加载
  - [x] SEO 标题、描述、关键词、Open Graph 信息更新
  - [x] 前台首屏资源按需加载、Element Plus 分包、静态资源缓存配置
- [x] 后台页面 `blog-cms`
  - [x] 登录、Dashboard
  - [x] 文章发布、编辑、列表管理
  - [x] 动态发布、编辑、列表管理
  - [x] 分类、标签、评论管理
  - [x] 站点设置、友链管理、关于页管理
  - [x] GitHub、又拍云、腾讯云图床配置与文件管理
  - [x] 账户修改、定时任务、任务日志
  - [x] 登录日志、操作日志、异常日志、访问日志
  - [x] 访客统计

### 优化中功能

- [~] 缓存策略：Redis 缓存已接入，缓存 Key、过期策略和主动失效策略持续整理中
- [~] 前端性能：首屏资源、Semantic UI 样式体积、Element Plus 分包、首页头图加载策略持续优化
- [~] 图床管理：GitHub 目录展示、空目录提示、错误诊断已优化，后续继续增强交互体验
- [~] 日志体系：stdout、滚动文件、请求链路信息已接入，后续可继续补充结构化检索与告警

### 未开发功能

- [ ] 插件化主题配置
- [ ] 多用户与细粒度 RBAC 权限
- [ ] 文章全文搜索引擎接入
- [ ] 评论审核工作流与垃圾评论拦截
- [ ] 后台批量导入/导出文章
- [ ] 自动化端到端测试与性能基准报告
- [ ] 更完整的部署脚本、Docker Compose 和 CI/CD 流程

## 隐藏功能

- 在前台访问 `/login` 路径登录后，可以以博主身份（带有博主标识）回复评论，且不需要填写昵称和邮箱即可提交
- 在 Markdown 中加入 `<meting-js server="netease" type="song" id="歌曲id" theme="#25CCF7"></meting-js>` （注意以正文形式添加，而不是代码片段）可以在文章中添加 [APlayer](https://github.com/DIYgod/APlayer) 音乐播放器，`netease` 为网易云音乐，其它配置及具体用法参考 [MetingJS](https://github.com/metowolf/MetingJS)
- 提供了两种隐藏文字效果：在 Markdown 中使用 `@@` 包住文字，文字会被渲染成"黑幕"效果，鼠标悬浮在上面时才会显示；使用 `%%` 包住文字，文字会被"蓝色覆盖层"遮盖，只有鼠标选中状态才会反色显示。例如：`@@隐藏文字@@`，`%%隐藏文字%%`

## LICENSE

[MIT](./LICENSE)

## Thanks

感谢 [Nblog](https://github.com/Naccl/NBlog) 项目中的前端页面

感谢 [grtblog](https://grtblog.js.org/) 项目的UI参考

感谢 [JetBrains](https://www.jetbrains.com/?from=DreamBlog) 提供的 Open Source License

感谢上面提到的每个开源项目
