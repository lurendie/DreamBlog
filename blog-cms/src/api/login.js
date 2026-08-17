import axios from '@/util/request'

export function login(loginForm) {
	return axios({
		url: 'login',
		method: 'POST',
		data: {
			...loginForm
		}
	})
}

// 调用后端注销接口，吊销 Redis 中的会话（改密/退出后旧 token 立即失效）
export function logout() {
	return axios({
		url: 'logout',
		method: 'POST'
	})
}
