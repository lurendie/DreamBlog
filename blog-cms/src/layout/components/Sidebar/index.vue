<template>
	<div :class="{'has-logo':showLogo}" class="sidebar-no-select">
		<logo v-if="showLogo" :collapse="isCollapse"/>
		<el-scrollbar wrap-class="scrollbar-wrapper">
			<el-menu
					:key="menuKey"
					:default-openeds="defaultOpeneds"
					:default-active="activeMenu"
					:collapse="isCollapse"
					:background-color="variables.menuBg"
					:text-color="variables.menuText"
					:unique-opened="true"
					:active-text-color="variables.menuActiveText"
					:collapse-transition="false"
					mode="vertical"
			>
				<sidebar-item v-for="route in routes" :key="route.path" :item="route" :base-path="route.path"/>
			</el-menu>
		</el-scrollbar>
	</div>
</template>

<script setup>
	import {computed} from 'vue'
	import {useRoute} from 'vue-router'
	import {storeToRefs} from 'pinia'
	import {routes} from '@/router'
	import {useAppStore, useSettingsStore} from '@/store'
	import Logo from './Logo'
	import SidebarItem from './SidebarItem'

	const route = useRoute()
	const {sidebar} = storeToRefs(useAppStore())
	const {sidebarLogo} = storeToRefs(useSettingsStore())
	const variables = {menuText: '#58677f', menuActiveText: '#0f766e', menuBg: 'rgba(255, 255, 255, 0.92)'}
	const defaultOpeneds = computed(() => route.matched.filter(item => item.path !== '/' && item.children?.length).map(item => item.path))
	const menuKey = computed(() => `${route.path}|${defaultOpeneds.value.join(',')}`)
	const activeMenu = computed(() => route.meta.activeMenu || route.path)
	const showLogo = computed(() => sidebarLogo.value)
	const isCollapse = computed(() => !sidebar.value.opened)
</script>

<style scoped>
	.sidebar-no-select {
		user-select: none;
	}
</style>
