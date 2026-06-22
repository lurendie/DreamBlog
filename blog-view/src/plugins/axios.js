import axios from "axios";
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

const request = axios.create({
	baseURL: import.meta.env.VITE_API_BASE_URL || import.meta.env.VUE_APP_API_BASE_URL || 'http://localhost:8090/blog/',
	timeout: 10000,
})

// 请求拦截
request.interceptors.request.use(
	config => {
		NProgress.start()
		const identification = window.localStorage.getItem('identification')
		//identification存在，且是基于baseURL的请求
		if (identification && !(config.url.startsWith('http://') || config.url.startsWith('https://'))) {
			config.headers.identification = identification
		}
		return config
	}
)

// 响应拦截
request.interceptors.response.use(
	response => {
		NProgress.done()
		const identification = response.headers?.identification
		if (identification) {
			//保存身份标识到localStorage
			window.localStorage.setItem('identification', identification)
		}
		return response.data
	},
	error => {
		NProgress.done()
		return Promise.reject(error)
	}
)

export default request
