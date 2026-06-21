import { createRouter, createWebHistory } from 'vue-router'
import store from '@/store'
import { updateSeo } from '@/util/seo'

const routes = [
	{
		path: '/login',
		component: () => import('@/views/Login.vue'),
		meta: {
			title: '登录',
			noindex: true,
		}
	},
	{
		path: '/',
		component: () => import('@/views/Index.vue'),
		redirect: '/home',
		children: [
			{
				path: '/home',
				name: 'home',
				component: () => import('@/views/home/Home.vue'),
				meta: {
					title: '首页',
				}
			},
			{
				path: '/archives',
				name: 'archives',
				component: () => import('@/views/archives/Archives.vue'),
				meta: {
					title: '归档',
				}
			},
			{
				path: '/blog/:id',
				name: 'blog',
				component: () => import('@/views/blog/Blog.vue'),
				meta: {
					title: '博客',
				}
			},
			{
				path: '/tag/:name',
				name: 'tag',
				component: () => import('@/views/tag/Tag.vue'),
				meta: {
					title: '标签',
				}
			},
			{
				path: '/category/:name',
				name: 'category',
				component: () => import('@/views/category/Category.vue'),
				meta: {
					title: '分类',
				}
			},
			{
				path: '/moments',
				name: 'moments',
				component: () => import('@/views/moments/Moments.vue'),
				meta: {
					title: '动态',
				}
			},
			{
				path: '/friends',
				name: 'friends',
				component: () => import('@/views/friends/Friends.vue'),
				meta: {
					title: '友人帐',
				}
			},
			{
				path: '/about',
				name: 'about',
				component: () => import('@/views/about/About.vue'),
				meta: {
					title: '关于我',
				}
			}
		]
	}
]

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes
})

//挂载路由守卫
router.beforeEach((to, from, next) => {
	updateSeo({
		title: to.meta.title,
		description: store.state.siteInfo?.siteDescription || '',
		keywords: store.state.siteInfo?.siteKeywords || '',
		path: to.fullPath,
		noindex: !!to.meta.noindex,
	})
	next()
})

export default router
