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

<script>
	import {mapGetters} from 'vuex'
	import {routes} from '@/router'
	import Logo from './Logo'
	import SidebarItem from './SidebarItem'

	const variables = {
		menuText: '#58677f',
		menuActiveText: '#0f766e',
		menuBg: 'rgba(255, 255, 255, 0.92)',
	}

	export default {
		components: {SidebarItem, Logo},
		computed: {
			...mapGetters([
				'sidebar'
			]),
			defaultOpeneds() {
				return this.$route.matched
					.filter(item => item.path !== '/' && item.children && item.children.length > 0)
					.map(item => item.path)
			},
			menuKey() {
				return `${this.$route.path}|${this.defaultOpeneds.join(',')}`
			},
			routes() {
				return routes
			},
			activeMenu() {
				const route = this.$route
				const {meta, path} = route
				// if set path, the sidebar will highlight the path you set
				if (meta.activeMenu) {
					return meta.activeMenu
				}
				return path
			},
			showLogo() {
				return this.$store.state.settings.sidebarLogo
			},
			variables() {
				return variables
			},
			isCollapse() {
				return !this.sidebar.opened
			}
		}
	}
</script>

<style scoped>
	.sidebar-no-select {
		user-select: none;
	}
</style>

