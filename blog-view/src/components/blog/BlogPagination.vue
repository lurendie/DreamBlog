<template>
	<div class="ui bottom" style="text-align:center">
		<LightPagination :current-page="pageNum" :page-count="totalPage" @current-change="handleCurrentChange"/>
	</div>
</template>

<script setup>
	import {ref, onActivated, getCurrentInstance} from 'vue'
	import {storeToRefs} from 'pinia'
	import {useRoute} from 'vue-router'
	import {useStore} from '@/store'
	import LightPagination from "@/components/common/LightPagination.vue"

	defineOptions({name: 'BlogPagination'})
	const props = defineProps({getBlogList: {type: Function, required: true}, totalPage: {type: Number, required: true}})
	const pageNum = ref(1)
	const route = useRoute()
	const {isBlogToHome, clientSize} = storeToRefs(useStore())
	const {proxy} = getCurrentInstance()
	onActivated(() => { if (!isBlogToHome.value) pageNum.value = 1 })
	function handleCurrentChange(newPage) {
		if (route.name === 'home') window.scrollTo({top: clientSize.value.clientHeight, behavior: 'smooth'})
		else proxy.scrollToTop()
		pageNum.value = newPage
		props.getBlogList(newPage)
	}
</script>

<style>
</style>
