# blog-view

## Project setup
``` 
npm install
```

### Environment variables
``` 
# .env.development / .env.production / .env.production.local
VUE_APP_API_BASE_URL=http://localhost:8090/blog/
VUE_APP_SITE_URL=http://localhost:8080
```

`VUE_APP_SITE_URL` 用于 canonical 与 Open Graph 输出。

站点地图与 robots 文本都可由后端 `blog-api` 动态提供，默认地址分别为 `/blog/sitemap.xml` 与 `/blog/robots.txt`。

注意：搜索引擎真正读取的 `robots.txt` 必须位于“站点根路径”。如果你的线上入口是前端静态站点根域名，那么仍需要让根路径 `/robots.txt` 可访问，或由网关把它转发到后端。

当前项目的 `npm run build` 会在构建前根据 `VUE_APP_SITE_URL` 与 `VUE_APP_API_BASE_URL` 自动生成前端根目录 `robots.txt`。

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
