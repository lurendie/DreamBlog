import moment from 'moment'

export function dateFormat(value, format = 'YYYY-MM-DD HH:mm:ss') {
	return moment(value).format(format)
}

export default {
	install(app) {
		app.config.globalProperties.dateFormat = dateFormat
	}
}
