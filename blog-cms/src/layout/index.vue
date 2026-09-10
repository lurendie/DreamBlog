<template>
  <div :class="classObj" class="app-wrapper">
    <div v-if="device==='mobile'&&sidebar.opened" class="drawer-bg" @click="handleClickOutside" />
    <sidebar class="sidebar-container" />
    <div class="main-container">
      <div :class="{'fixed-header':fixedHeader}">
        <navbar />
      </div>
      <app-main />
    </div>
  </div>
</template>

<script setup>
import {computed, onBeforeUnmount, onMounted, watch} from 'vue'
import {useRoute} from 'vue-router'
import {storeToRefs} from 'pinia'
import {Navbar, Sidebar, AppMain} from './components'
import {useAppStore, useSettingsStore} from '@/store'

const route = useRoute()
const appStore = useAppStore()
const settingsStore = useSettingsStore()
const {sidebar, device} = storeToRefs(appStore)
const {fixedHeader} = storeToRefs(settingsStore)
const classObj = computed(() => ({
  hideSidebar: !sidebar.value.opened,
  openSidebar: sidebar.value.opened,
  withoutAnimation: sidebar.value.withoutAnimation,
  mobile: device.value === 'mobile'
}))
const isMobile = () => document.body.getBoundingClientRect().width - 1 < 992
const resizeHandler = () => {
  if (!document.hidden) {
    const mobile = isMobile()
    appStore.toggleDevice(mobile ? 'mobile' : 'desktop')
    if (mobile) appStore.closeSideBar({withoutAnimation: true})
  }
}
const handleClickOutside = () => appStore.closeSideBar({withoutAnimation: false})
watch(() => route.fullPath, () => {
  if (device.value === 'mobile' && sidebar.value.opened) handleClickOutside()
})
onMounted(() => {
  window.addEventListener('resize', resizeHandler)
  if (isMobile()) {
    appStore.toggleDevice('mobile')
    appStore.closeSideBar({withoutAnimation: true})
  }
})
onBeforeUnmount(() => window.removeEventListener('resize', resizeHandler))
</script>

<style lang="scss" scoped>
  @use "@/assets/styles/mixin.scss" as mixins;
  @use "@/assets/styles/variables.scss" as variables;

  .app-wrapper {
    @include mixins.clearfix;
    position: relative;
    height: 100%;
    width: 100%;
    &.mobile.openSidebar{
      position: fixed;
      top: 0;
    }
  }
  .drawer-bg {
    background: #000;
    opacity: 0.3;
    width: 100%;
    top: 0;
    height: 100%;
    position: absolute;
    z-index: 1702;
  }

  .fixed-header {
    position: fixed;
    top: 0;
    right: 0;
    z-index: 1701;
    width: calc(100% - #{variables.$sideBarWidth});
    transition: width 0.28s;
  }

  .hideSidebar .fixed-header {
    width: calc(100% - 54px)
  }

  .mobile .fixed-header {
    width: 100%;
  }
</style>


