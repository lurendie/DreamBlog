import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import 'dayjs/locale/zh-cn'

//dayjs 国际化（替代已停止维护的 moment）
dayjs.extend(relativeTime)
dayjs.locale('zh-cn')

export function dateFormat(value, format = 'YYYY-MM-DD HH:mm:ss') {
	return dayjs(value).format(format)
}

export function dateFromNow(value) {
	//相对时间大于一个月，显示详细时间
	if (dayjs().diff(dayjs(value)) > 2592000000) {
		return dayjs(value).format('YYYY-MM-DD HH:mm')
	}
	return dayjs(value).fromNow()
}

export default {
	install(app) {
		app.config.globalProperties.dateFormat = dateFormat
		app.config.globalProperties.dateFromNow = dateFromNow
	}
}