import {createPinia} from 'pinia'
import {useAppStore} from './modules/app'
import {useSettingsStore} from './modules/settings'

export const pinia = createPinia()
export {useAppStore, useSettingsStore}

const app = useAppStore(pinia)
const settings = useSettingsStore(pinia)
const store = {
	state: {app: app.$state, settings: settings.$state},
	dispatch(type, payload) {
		const [module, action] = type.split('/')
		const target = module === 'app' ? app : settings
		if (typeof target[action] !== 'function') throw new Error(`Unknown store action: ${type}`)
		return target[action](payload)
	},
}
export default store
