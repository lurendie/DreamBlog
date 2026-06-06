import Vue from 'vue'
import VueRouter from 'vue-router'
import { updateSeo } from '@/util/seo'

Vue.use(VueRouter)

const routes = [
	{
		path: '/login',
		component: () => import('@/views/Login'),
		meta: {
			title: '登录',
			description: '登录后台管理或使用博主身份参与站点互动。',
			noindex: true,
		}
	},
	{
		path: '/',
		component: () => import('@/views/Index'),
		redirect: '/home',
		children: [
			{
				path: '/home',
				name: 'home',
				component: () => import('@/views/home/Home'),
				meta: {
					title: '首页',
					description: '浏览最新博客文章、精选内容与站点概览。',
				}
			},
			{
				path: '/archives',
				name: 'archives',
				component: () => import('@/views/archives/Archives'),
				meta: {
					title: '归档',
					description: '按时间归档浏览全部文章内容。',
				}
			},
			{
				path: '/blog/:id',
				name: 'blog',
				component: () => import('@/views/blog/Blog'),
				meta: {
					title: '博客',
					description: '查看博客文章正文、分类、标签与评论。',
				}
			},
			{
				path: '/tag/:name',
				name: 'tag',
				component: () => import('@/views/tag/Tag'),
				meta: {
					title: '标签',
					description: '按标签聚合浏览相关博客文章。',
				}
			},
			{
				path: '/category/:name',
				name: 'category',
				component: () => import('@/views/category/Category'),
				meta: {
					title: '分类',
					description: '按分类聚合浏览相关博客文章。',
				}
			},
			{
				path: '/moments',
				name: 'moments',
				component: () => import('@/views/moments/Moments'),
				meta: {
					title: '动态',
					description: '查看博主发布的动态内容与日常记录。',
				}
			},
			{
				path: '/friends',
				name: 'friends',
				component: () => import('@/views/friends/Friends'),
				meta: {
					title: '友人帐',
					description: '查看站点友链、站点说明与相关评论内容。',
				}
			},
			{
				path: '/about',
				name: 'about',
				component: () => import('@/views/about/About'),
				meta: {
					title: '关于我',
					description: '了解站长介绍、个人信息与关于页面内容。',
				}
			}
		]
	}
]

const router = new VueRouter({
	mode: 'history',
	base: process.env.BASE_URL,
	routes
})

//挂载路由守卫
router.beforeEach((to, from, next) => {
	updateSeo({
		title: to.meta.title,
		description: to.meta.description,
		path: to.fullPath,
		noindex: !!to.meta.noindex,
	})
	next()
})

export default router
