import axios from 'axios'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'
import {Message} from 'element-ui'
import {clearLoginState, getStoredUser} from '@/util/storage'
import router from '@/router'

const request = axios.create({
	baseURL: process.env.VUE_APP_ADMIN_API_BASE_URL || 'http://localhost:8090/admin/',
	timeout: 5000
})

let CancelToken = axios.CancelToken

function redirectToLogin() {
	if (router.currentRoute.path !== '/login') {
		router.push('/login').catch(() => {})
	}
}

// 请求拦截
request.interceptors.request.use(config => {
		//对于访客模式，除GET请求外，都拦截并提示
		const user = getStoredUser()
		if (user && user.role !== 'ROLE_admin' && config.method !== 'get') {
			config.cancelToken = new CancelToken(function executor(cancel) {
				cancel('演示模式，不允许操作')
			})
			return config
		}

		NProgress.start()
		const token = window.localStorage.getItem('token')
		if (token) {
			config.headers.Authorization = token
		}
		return config
	},
	error => {
		console.info(error)
		return Promise.reject(error)
	}
)

// 响应拦截
request.interceptors.response.use(response => {
		NProgress.done()
		const res = response.data
		if (response.status === 401 || response.status === 403 || res.code === 401 || res.code === 403) {
			clearLoginState()
			redirectToLogin()
		}
		if (res.code !== 200) {
			let msg = res.msg || 'Error'
			Message.error(msg)
			return Promise.reject(new Error(msg))
		}
		return res
	},
	error => {
		console.info(error)
		NProgress.done()
		if (error.response && (error.response.status === 401 || error.response.status === 403)) {
			clearLoginState()
			redirectToLogin()
		}
		Message.error(error.message)
		return Promise.reject(error)
	}
)

export default request
