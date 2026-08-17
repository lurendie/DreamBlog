import dayjs from 'dayjs'

//dayjs 国际化（替代已停止维护的 moment）
dayjs.locale('zh-cn')

export function dateFormat(value, format = 'YYYY-MM-DD HH:mm:ss') {
	return dayjs(value).format(format)
}

export default {
	install(app) {
		app.config.globalProperties.dateFormat = dateFormat
	}
}