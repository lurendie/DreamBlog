<template>
	<div v-if="!item.hidden">
		<template v-if="hasOneShowingChild(item.children,item) && (!onlyOneChild.children||onlyOneChild.noShowingChildren)&&!item.alwaysShow">
			<app-link v-if="onlyOneChild.meta" :to="resolvePath(onlyOneChild.path)">
				<el-menu-item :index="resolvePath(onlyOneChild.path)" :class="{'submenu-title-noDropdown':!isNest}">
					<item :icon="onlyOneChild.meta.icon||(item.meta&&item.meta.icon)" :title="onlyOneChild.meta.title"/>
				</el-menu-item>
			</app-link>
		</template>

			<el-sub-menu v-else ref="subMenu" :index="resolvePath(item.path)" popper-append-to-body>
			<template #title>
				<item v-if="item.meta" :icon="item.meta && item.meta.icon" :title="item.meta.title"/>
			</template>
			<sidebar-item
					v-for="child in item.children"
					:key="child.path"
					:is-nest="true"
					:item="child"
					:base-path="resolvePath(child.path)"
					class="nest-menu"
			/>
			</el-sub-menu>
	</div>
</template>

<script setup>
	import {ref} from 'vue'
	import {isExternal} from '@/util/validate'
	import Item from './Item'
	import AppLink from './Link'

	defineOptions({name: 'SidebarItem'})
	const props = defineProps({item: {type: Object, required: true}, isNest: Boolean, basePath: {type: String, default: ''}})
	const onlyOneChild = ref(null)
	function hasOneShowingChild(children = [], parent) {
		const showingChildren = children.filter(child => { if (child.hidden) return false; onlyOneChild.value = child; return true })
		if (showingChildren.length === 1) return showingChildren[0].name === 'Dashboard'
		if (showingChildren.length === 0) { onlyOneChild.value = {...parent, path: '', noShowingChildren: true}; return true }
		return false
	}
	function resolvePath(routePath) {
		if (isExternal(routePath)) return routePath
		if (isExternal(props.basePath)) return props.basePath
		return `/${[props.basePath, routePath].join('/').split('/').filter(Boolean).join('/')}`
	}
</script>


