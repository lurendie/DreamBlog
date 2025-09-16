import axios from '@/plugins/axios'

export function getMomentListByPageNum(token, pageNum) {
	if(token ===null || token === undefined || token === '') {
		return axios({
		url: 'moments',
		method: 'GET',
		params: {
			pageNum
		}
		})
	}
	return axios({
		url: 'moments',
		method: 'GET',
		headers: {
			Authorization: token,
		},
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