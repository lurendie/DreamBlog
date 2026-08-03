import SvgIcon from '@/components/SvgIcon'// svg component
import 'virtual:svg-icons-register'
import { h } from 'vue'
import {
	AlarmClock,
	ArrowDown,
	ArrowRight,
	ChatDotRound,
	Check,
	Close,
	CopyDocument,
	DataAnalysis,
	Delete,
	Document,
	DocumentChecked,
	DocumentCopy,
	DocumentDelete,
	Edit,
	Finished,
	FolderOpened,
	Link,
	Menu,
	Picture,
	Plus,
	QuestionFilled,
	Search,
	SetUp,
	Tickets,
	Tools,
	Upload,
	UserFilled,
	View,
	TrendCharts,
	Comment,
	Opportunity,
	List,
	Histogram,
} from '@element-plus/icons-vue'

function createLegacyIcon(component) {
	return {
		name: 'LegacyElementIcon',
		render() {
			return h(component)
		}
	}
}

const legacyIconMap = {
	'el-icon-alarm-clock': AlarmClock,
	'el-icon-caret-bottom': ArrowDown,
	'el-icon-caret-right': ArrowRight,
	'el-icon-chat-dot-round': ChatDotRound,
	'el-icon-check': Check,
	'el-icon-close': Close,
	'el-icon-data-line': DataAnalysis,
	'el-icon-delete': Delete,
	'el-icon-document': Document,
	'el-icon-document-checked': DocumentChecked,
	'el-icon-document-copy': DocumentCopy,
	'el-icon-document-delete': DocumentDelete,
	'el-icon-edit': Edit,
	'el-icon-finished': Finished,
	'el-icon-folder-opened': FolderOpened,
	'el-icon-link': Link,
	'el-icon-menu': Menu,
	'el-icon-picture': Picture,
	'el-icon-plus': Plus,
	'el-icon-question': QuestionFilled,
	'el-icon-s-comment': Comment,
	'el-icon-s-data': Histogram,
	'el-icon-s-marketing': TrendCharts,
	'el-icon-s-opportunity': Opportunity,
	'el-icon-s-order': List,
	'el-icon-s-tools': Tools,
	'el-icon-search': Search,
	'el-icon-setting': SetUp,
	'el-icon-tickets': Tickets,
	'el-icon-upload': Upload,
	'el-icon-user-solid': UserFilled,
	'el-icon-view': View,
}

export default {
	install(app) {
		app.component('svg-icon', SvgIcon)
		Object.entries(legacyIconMap).forEach(([name, component]) => {
			app.component(name, createLegacyIcon(component))
		})
	}
}
