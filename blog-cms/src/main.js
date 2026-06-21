import {createApp} from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

//normalize.css
import 'normalize.css/normalize.css' // A modern alternative to CSS resets
//element-plus
import ElementPlus, {ElMessage, ElMessageBox} from 'element-plus'
import 'element-plus/dist/index.css'
//global css
import '@/assets/styles/index.scss'
//icon
import Icons from '@/icons'

//moment
import DateTimeFormatUtils from './util/dateTimeFormatUtils.js'
//md-editor-v3
import {MdEditor} from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
//v-viewer
import 'viewerjs/dist/viewer.css'
import Viewer from 'v-viewer'
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
	.use(ElementPlus)
	.use(MdEditor)
	.use(Viewer)
	.use(Icons)
	.use(Directives)
	.use(DateTimeFormatUtils)
	.mount('#app')
