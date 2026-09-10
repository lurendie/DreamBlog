import {reactive, toRefs} from 'vue'
import {defineStore} from 'pinia'
import defaultSettings from '@/settings'

export const useSettingsStore = defineStore('settings', () => {
	const state = reactive({
		title: defaultSettings.title,
		logo: defaultSettings.logo,
		fixedHeader: defaultSettings.fixedHeader,
		sidebarLogo: defaultSettings.sidebarLogo,
		defaultOpeneds: defaultSettings.defaultOpeneds,
	})
	function changeSetting({key, value} = {}) {
		if (Object.prototype.hasOwnProperty.call(state, key)) state[key] = value
	}
	return {...toRefs(state), changeSetting}
})

