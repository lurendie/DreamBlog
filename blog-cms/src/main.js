import {createApp} from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

//normalize.css
import 'normalize.css/normalize.css' // A modern alternative to CSS resets
//element-plus
import {ElMessage, ElMessageBox} from 'element-plus'
import 'element-plus/es/components/message/style/css'
import 'element-plus/es/components/message-box/style/css'
//global css
import '@/assets/styles/index.scss'
//icon
import Icons from '@/icons'

//moment
import DateTimeFormatUtils from './util/dateTimeFormatUtils.js'
// directive
import Directives from './util/directive'

const app = createApp(App)

app.config.globalProperties.msgSuccess = function (msg) {
	ElMessage.success(msg)
}

app.config.globalProperties.msgError = function (msg) {
	ElMessage.error(msg)
}

app.config.globalProperties.$message = ElMessage
app.config.globalProperties.$confirm = ElMessageBox.confirm

app
	.use(store)
	.use(router)
	.use(Icons)
	.use(Directives)
	.use(DateTimeFormatUtils)
	.mount('#app')
