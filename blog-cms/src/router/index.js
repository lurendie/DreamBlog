import {createRouter, createWebHistory} from 'vue-router'
import getPageTitle from '@/util/get-page-title'
import Layout from '@/layout'
import {clearLoginState, getStoredUser} from '@/util/storage'

const routes = [
	{
		path: '/404',
		component: () => import('@/views/404.vue'),
		meta: {title: '404 NOT FOUND'},
		hidden: true
	},
	{
		path: '/login',
		component: () => import('@/views/login/index.vue'),
		meta: {title: '后台管理登录'},
		hidden: true
	},
	{
		path: '/',
		component: Layout,
		redirect: '/dashboard',
		children: [
			{
				path: 'dashboard',
				name: 'Dashboard',
				component: () => import('@/views/dashboard/index.vue'),
				meta: {title: 'Dashboard', icon: 'dashboard'}
			}
		]
	},
	{
		path: '/blog',
		name: 'Blog',
		redirect: '/blog/write',
		component: Layout,
		meta: {title: '博客管理', icon: 'el-icon-menu'},
		children: [
			{
				path: 'write',
				name: 'WriteBlog',
				component: () => import('@/views/blog/blog/WriteBlog.vue'),
				meta: {title: '写文章', icon: 'el-icon-edit'}
			},
			{
				path: 'moment/write',
				name: 'WriteMoment',
				component: () => import('@/views/blog/moment/WriteMoment.vue'),
				meta: {title: '写动态', icon: 'el-icon-edit'}
			},
			{
				path: 'edit/:id',
				name: 'EditBlog',
				component: () => import('@/views/blog/blog/WriteBlog.vue'),
				meta: {title: '编辑文章', icon: 'el-icon-edit'},
				hidden: true
			},
			{
				path: 'moment/edit/:id',
				name: 'EditMoment',
				component: () => import('@/views/blog/moment/WriteMoment.vue'),
				meta: {title: '编辑动态', icon: 'el-icon-edit'},
				hidden: true
			},
			{
				path: 'list',
				name: 'BlogList',
				component: () => import('@/views/blog/blog/BlogList.vue'),
				meta: {title: '文章管理', icon: 'el-icon-s-order'}
			},
			{
				path: 'moment/list',
				name: 'MomentList',
				component: () => import('@/views/blog/moment/MomentList.vue'),
				meta: {title: '动态管理', icon: 'el-icon-chat-dot-round'}
			},
			{
				path: 'category/list',
				name: 'CategoryList',
				component: () => import('@/views/blog/category/CategoryList.vue'),
				meta: {title: '分类管理', icon: 'el-icon-s-opportunity'}
			},
			{
				path: 'tag/list',
				name: 'TagList',
				component: () => import('@/views/blog/tag/TagList.vue'),
				meta: {title: '标签管理', icon: 'biaoqian'}
			},
			{
				path: 'comment/list',
				name: 'CommentList',
				component: () => import('@/views/blog/comment/CommentList.vue'),
				meta: {title: '评论管理', icon: 'el-icon-s-comment'}
			},
		]
	},
	{
		path: '/page',
		name: 'Page',
		redirect: '/page/site',
		component: Layout,
		meta: {title: '页面管理', icon: 'el-icon-document-copy'},
		children: [
			{
				path: 'site',
				name: 'SiteSetting',
				component: () => import('@/views/page/SiteSetting.vue'),
				meta: {title: '站点设置', icon: 'bianjizhandian'}
			},
			{
				path: 'friend',
				name: 'FriendList',
				component: () => import('@/views/page/FriendList.vue'),
				meta: {title: '友链管理', icon: 'friend'}
			},
			{
				path: 'about',
				name: 'About',
				component: () => import('@/views/page/About.vue'),
				meta: {title: '关于我', icon: 'el-icon-tickets'}
			},
		]
	},
	{
		path: '/pictureHosting',
		name: 'PictureHosting',
		redirect: '/pictureHosting/setting',
		component: Layout,
		meta: {title: '图床管理', icon: 'el-icon-picture'},
		children: [
			{
				path: 'setting',
				name: 'Setting',
				component: () => import('@/views/pictureHosting/Setting.vue'),
				meta: {title: '配置', icon: 'el-icon-setting'}
			},
			{
				path: 'github',
				name: 'GithubManage',
				component: () => import('@/views/pictureHosting/GithubManage.vue'),
				meta: {title: 'GitHub', icon: 'el-icon-folder-opened'}
			},
			{
				path: 'upyun',
				name: 'UpyunManage',
				component: () => import('@/views/pictureHosting/UpyunManage.vue'),
				meta: {title: '又拍云', icon: 'el-icon-folder-opened'}
			},
			{
				path: 'txyun',
				name: 'TxyunManage',
				component: () => import('@/views/pictureHosting/TxyunManage.vue'),
				meta: {title: '腾讯云', icon: 'el-icon-folder-opened'}
			},
		]
	},
	{
		path: '/system',
		name: 'System',
		redirect: '/system/account',
		component: Layout,
		meta: {title: '系统管理', icon: 'el-icon-s-tools'},
		children: [
			{
				path: 'account',
				name: 'Account',
				component: () => import('@/views/system/Account.vue'),
				meta: {title: '修改账户', icon: 'el-icon-user-solid'}
			},
			{
				path: 'job',
				name: 'JobList',
				component: () => import('@/views/system/ScheduleJobList.vue'),
				meta: {title: '定时任务', icon: 'el-icon-alarm-clock'}
			},
		]
	},
	{
		path: '/log',
		name: 'Log',
		redirect: '/log/job',
		component: Layout,
		meta: {title: '日志管理', icon: 'el-icon-document'},
		children: [
			{
				path: 'job',
				name: 'JobLog',
				component: () => import('@/views/log/ScheduleJobLog.vue'),
				meta: {title: '任务日志', icon: 'el-icon-alarm-clock'}
			},
			{
				path: 'login',
				name: 'LoginLog',
				component: () => import('@/views/log/LoginLog.vue'),
				meta: {title: '登录日志', icon: 'el-icon-finished'}
			},
			{
				path: 'operation',
				name: 'OperationLog',
				component: () => import('@/views/log/OperationLog.vue'),
				meta: {title: '操作日志', icon: 'el-icon-document-checked'}
			},
			{
				path: 'exception',
				name: 'ExceptionLog',
				component: () => import('@/views/log/ExceptionLog.vue'),
				meta: {title: '异常日志', icon: 'el-icon-document-delete'}
			},
			{
				path: 'visit',
				name: 'VisitLog',
				component: () => import('@/views/log/VisitLog.vue'),
				meta: {title: '访问日志', icon: 'el-icon-data-line'}
			},
		]
	},
	{
		path: '/statistics',
		name: 'Statistics',
		redirect: '/statistics/visitor',
		component: Layout,
		meta: {title: '数据统计', icon: 'el-icon-s-data'},
		children: [
			{
				path: 'visitor',
				name: 'Visitor',
				component: () => import('@/views/statistics/Visitor.vue'),
				meta: {title: '访客统计', icon: 'el-icon-s-marketing'}
			},
		]
	},

	// 404 page must be placed at the end !!!
	{path: '/:pathMatch(.*)*', redirect: '/404', hidden: true}
]

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes
})

//挂载路由守卫
router.beforeEach((to, from, next) => {
	if (to.path !== '/login') {
		//获取token
		const tokenStr = window.localStorage.getItem('token')
		const user = getStoredUser()
		if (!tokenStr || !user) {
			clearLoginState()
			return next("/login")
		}
	}
	document.title = getPageTitle(to.meta.title)
	next()
})

export default router
export {routes}

