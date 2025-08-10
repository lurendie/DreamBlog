<p align="center">
	<a href="https://naccl.top/" target="_blank">
		<img src="./pic/logo.jpg" alt="zero_blog logo" style="width: 100px; height: 100px">
	</a>
</p>
<p align="center">
	<img src="https://img.shields.io/badge/RUST-1.78-orange">
	<img src="https://img.shields.io/badge/actix-web">
	<img src="https://img.shields.io/badge/Vue-2.6.11-brightgreen">
	<img src="https://img.shields.io/badge/license-MIT-blue">
	<!-- <img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FNaccl%2FNBlog&count_bg=%2344CC11&title_bg=%23555555&icon=notist.svg&icon_color=%231296DB&title=hits&edge_flat=false"> -->
</p>

# 介绍 Introduce
  ZeroBlog基于actix-web+Vue 博客系统,自用博客，长期维护中，现阶段`开发阶段`，项目以学习RUST为主。

# 开发环境 Development Deploy 
  1. `git clone` 项目 
  2. 创建 `MySQL` 数据库zero_blog，并执行sql文件初始化表数据
  3. 安装 `Redis` 并启动
  4. 修改配置文件`config.yaml`确认MySQL和Redis数据库用户密码
  5. `cargo run` 启动后端服务 
  6. 分别在blog-cms和blog-view目录下执行`npm install`安装依赖
  7. 分别在blog-cms和blog-view目录下执行`npm run serve`启动前后台页面



# TODO 
 - 日志系统 [ 完成 ]
 - Sea-orm or Mysql [ 完成 ]
 - 前台页面
   - 首页 [ 完成 ]
   - 关于 [ 完成 ]
   - 分类 [ 完成 ]
   - 标签 [ 完成 ]
   - 友链 [ 完成 ]
   - 归档 [ 完成 ]
   - 动态 [ 完成 ]
   - 评论 [ 完成 ]
   - JWT  [ 完成 ]
   - Redis or cache DataBase [进行中]
 - 后台页面
   - 首页 [ 进行中 ]
   - 分类 [ 进行中 ]
   - 标签 [ 进行中 ]
   - 友链 [ 进行中 ]
   - 文章 [ 进行中 ]
   - 动态 [ 进行中 ]
   - JWT  [ 完成 ]
  
# Thanks
 感谢[Nblog](https://github.com/Naccl/NBlog)的前端页面
 
 感谢 JetBrains 提供的 Open Source License