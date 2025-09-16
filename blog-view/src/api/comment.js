import axios from '@/plugins/axios'

export function getCommentListByQuery(token, query) {
	if(token ===null || token === undefined || token === '') {
		return axios({
		url: 'comments',
		method: 'GET',
		params: {
			...query
		}
		})
	}
	return axios({
		url: 'comments',
		method: 'GET',
		headers: {
			Authorization: token,
		},
		params: {
			...query
		}
	})
}

export function submitComment(token, form) {
	if(token ===null || token === undefined || token === '') {
		return axios({
		url: 'comment',
		method: 'POST',
		data: {
			...form
		}
		})
	}
	return axios({
		url: 'comment',
		method: 'POST',
		headers: {
			Authorization: token,
		},
		data: {
			...form
		}
	})
}