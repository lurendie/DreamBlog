import SvgIcon from '@/components/SvgIcon'// svg component
import 'virtual:svg-icons-register'

export default {
	install(app) {
		app.component('svg-icon', SvgIcon)
	}
}
