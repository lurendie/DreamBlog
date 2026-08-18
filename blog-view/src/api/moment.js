import axios from '@/plugins/axios'

export function getMomentListByPageNum(token, pageNum) {
	const headers = token ? {Authorization: token} : {}
	return axios({
		url: 'moments',
		method: 'GET',
		headers,
		params: {
			pageNum
		}
	})
}

export function likeMoment(id) {
	return axios({
		url: `moment/like/${id}`,
		method: 'POST',
	})
}