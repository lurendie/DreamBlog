import {ref} from 'vue'
import {defineStore} from 'pinia'

export const useAppStore = defineStore('app', () => {
  const sidebar = ref({opened: true, withoutAnimation: false})
  const device = ref('desktop')
  function toggleSideBar() { sidebar.value.opened = !sidebar.value.opened; sidebar.value.withoutAnimation = false }
  function closeSideBar({withoutAnimation} = {}) { sidebar.value.opened = false; sidebar.value.withoutAnimation = withoutAnimation }
  function toggleDevice(value) { device.value = value }
  return {sidebar, device, toggleSideBar, closeSideBar, toggleDevice}
})
