import axios from '@/plugins/axios'

export function getCommentListByQuery(token, query) {
	const headers = token ? {Authorization: token} : {}
	return axios({
		url: 'comments',
		method: 'GET',
		headers,
		params: {
			...query
		}
	})
}

export function submitComment(token, form) {
	const headers = token ? {Authorization: token} : {}
	return axios({
		url: 'comment',
		method: 'POST',
		headers,
		data: {
			...form
		}
	})
}