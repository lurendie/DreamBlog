import axios from 'axios'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'
import {ElMessage} from 'element-plus'
import {clearLoginState, getStoredUser} from '@/util/storage'
import router from '@/router'

const request = axios.create({
	baseURL: import.meta.env.VITE_ADMIN_API_BASE_URL || import.meta.env.VUE_APP_ADMIN_API_BASE_URL || 'http://localhost:8090/admin/',
	timeout: 5000
})

//后端约定：业务失败时统一返回 HTTP 200 + body.code 错误码；仅当发生 JWT 相关错误时为 401/403/502。
//以下统一走"清登录态 + 跳登录"。
const INVALID_SESSION_CODES = [401, 403, 502]

function handleInvalidSession() {
	clearLoginState()
	NProgress.done()
	redirectToLogin()
}

function redirectToLogin() {
	if (router.currentRoute.value.path !== '/login') {
		router.push('/login').catch(() => {})
	}
}

// 请求拦截
request.interceptors.request.use(config => {
		//对于访客模式，除GET请求外，都拦截并提示
		const user = getStoredUser()
		if (user && user.role !== 'ROLE_admin' && config.method !== 'get') {
			// AbortController 取消请求（axios 0.22+ / 1.x 均通过 config.signal 支持）
			const controller = new AbortController()
			config.signal = controller.signal
			controller.abort('演示模式，不允许操作')
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
		return Promise.reject(error)
	}
)

// 响应拦截
request.interceptors.response.use(response => {
		NProgress.done()
		const res = response.data
		//后端恒返回 HTTP 200，业务错误码写在 res.code 中；401/403/502 视为 JWT 失效，清登录态并跳登录
		if (res.code !== undefined && INVALID_SESSION_CODES.includes(res.code)) {
			handleInvalidSession()
			return Promise.reject(new Error(res.msg || '登录状态已失效'))
		}
		if (res.code !== 200) {
			let msg = res.msg || 'Error'
			ElMessage.error(msg)
			return Promise.reject(new Error(msg))
		}
		return res
	},
	error => {
		NProgress.done()
		//真实的 HTTP 层错误（非业务 code），遇到 401/403/502 同样视为 JWT 失效
		if (error.response && error.response.status && INVALID_SESSION_CODES.includes(error.response.status)) {
			handleInvalidSession()
		}
		ElMessage.error(error.message)
		return Promise.reject(error)
	}
)

export default request
