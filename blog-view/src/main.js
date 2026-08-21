import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
//自定义css
import './assets/css/base.css'
//阿里icon
import './assets/css/icon/iconfont.css'
//typo.css
import "./assets/css/typo.css";
//semantic-ui
import 'semantic-ui-css/components/reset.min.css'
import 'semantic-ui-css/components/site.min.css'
import 'semantic-ui-css/components/container.min.css'
import 'semantic-ui-css/components/grid.min.css'
import 'semantic-ui-css/components/segment.min.css'
import 'semantic-ui-css/components/menu.min.css'
import 'semantic-ui-css/components/header.min.css'
import 'semantic-ui-css/components/list.min.css'
import 'semantic-ui-css/components/item.min.css'
import 'semantic-ui-css/components/label.min.css'
import 'semantic-ui-css/components/divider.min.css'
import 'semantic-ui-css/components/image.min.css'
import 'semantic-ui-css/components/card.min.css'
import 'semantic-ui-css/components/comment.min.css'
import 'semantic-ui-css/components/message.min.css'
import 'semantic-ui-css/components/icon.min.css'
//moment
import dateTimeFormatUtils from './util/dateTimeFormatUtils.js'
//directive
import directives from './util/directive'
import { lazyMessage, lazyNotify, showMessage } from './util/feedback'

const app = createApp(App)

//set canonical link：使用构建期变量注入站点地址，避免 index.html 中硬编码 localhost
;(function injectCanonical() {
	const siteUrl = import.meta.env.VITE_SITE_URL || import.meta.env.VUE_APP_SITE_URL
	if (!siteUrl) {
		return
	}
	let canonical = document.querySelector('link[rel="canonical"]')
	if (!canonical) {
		canonical = document.createElement('link')
		canonical.rel = 'canonical'
		document.head.appendChild(canonical)
	}
	canonical.setAttribute('href', siteUrl)
})()

app.use(dateTimeFormatUtils)
app.use(directives)
app.use(router)
app.use(store)

app.config.globalProperties.msgSuccess = function (msg) {
	showMessage('success', msg)
}

app.config.globalProperties.msgError = function (msg) {
	showMessage('error', msg)
}

app.config.globalProperties.msgInfo = function (msg) {
	showMessage('info', msg);
}
app.config.globalProperties.$message = lazyMessage
app.config.globalProperties.$notify = lazyNotify

const cubic = value => Math.pow(value, 3);
const easeInOutCubic = value => value < 0.5 ? cubic(value * 2) / 2 : 1 - cubic((1 - value) * 2) / 2;
//滚动至页面顶部，使用 Element Plus 回到顶部组件中的算法
app.config.globalProperties.scrollToTop = function () {
	const el = document.documentElement
	const beginTime = Date.now()
	const beginValue = el.scrollTop
	const rAF = window.requestAnimationFrame || (func => setTimeout(func, 16))
	const frameFunc = () => {
		const progress = (Date.now() - beginTime) / 500;
		if (progress < 1) {
			el.scrollTop = beginValue * (1 - easeInOutCubic(progress))
			rAF(frameFunc)
		} else {
			el.scrollTop = 0
		}
	}
	rAF(frameFunc)
}
app.mount('#app')
